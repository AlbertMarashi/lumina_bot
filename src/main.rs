mod pinecone_lib;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
mod utils;

use openai::chat::{ChatCompletionMessageRole, ChatCompletionMessage};
use openai::set_key;
use pinecone_lib::VectorQueryMatch;
use serde::Serialize;
use serde_json::json;
use serenity::builder::{CreateInputText, CreateMessage};
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::http::error::{ErrorResponse, DiscordJsonError};
use serenity::http::{Http, self, CacheHttp};
use serenity::model::prelude::{Message, ChannelId, Channel, MessageFlags, Interaction};
use serenity::model::prelude::component::{ButtonStyle, InputTextStyle};
use serenity::prelude::{Context, GatewayIntents, HttpError};
use serenity::{prelude::EventHandler, async_trait};
use serenity::framework::{StandardFramework, standard::macros::group};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

const BOT_ID: u64 = 900184529623474237;
const BOT_USER_ID: u64 = 1014851190472835082;
const MODEL: &'static str = "gpt-3.5-turbo";

async fn start_running(http: Arc<Http>) -> Result<(), anyhow::Error>{
    Ok(())
}

#[group]
#[commands(ping)]
struct General;

struct Handler;

// get the groupings of messages from the same user and return them in chronological order
async fn get_user_messages(msg: &Message, ctx: &Context) -> Vec<ChatCompletionMessage> {
    // get the 20 previous messages from the channel
    let mut messages = msg
        .channel_id
        .messages(&ctx.http, |retriever| retriever.before(msg.id).limit(30))
        .await
        .unwrap()
        .into_iter();

    // turn array of messages into Array<Array<Message>> where each inner array, is a grouping of messages
    // from the same order, in chronological order

    let mut messages_by_order: Vec<Vec<Message>> = Vec::new();
    // push the first message into the first group
    messages_by_order.push(Vec::from([msg.clone()]));

    // use a while loop here of the above code
    while let Some(message) = messages.next() {
        let last_group = messages_by_order.last_mut().unwrap();

        let last_message = last_group.last().unwrap();

        if last_message.author.id == message.author.id {
            last_group.push(message.clone());
        } else {
            let mut new_last_group = Vec::new();
            new_last_group.push(message.clone());
            messages_by_order.push(new_last_group);
        }
    }

    let mut messages: Vec<Message> = messages_by_order
        .into_iter()
        .map(|group| {
            let mut iter = group.into_iter().rev();
            let mut first = iter.next().unwrap();
            while let Some(message) = iter.next() {
                first.content.push_str("\n");
                first.content.push_str(&message.content);
            }
            first
        })
        .collect();

    // use only the last 10 groups of messages
    messages.truncate(10);

    // reverse the messages so that the oldest message is first
    messages.reverse();


    // convert the messages into ChatCompletionMessages
    let mut chat_gpt_messages = Vec::new();

    // convert the above into a loop that pushes messages into chat_gpt_messages
    for message in messages {
        chat_gpt_messages.push(match message.author.id == BOT_USER_ID {
            true => ChatCompletionMessage {
                content: message.content.clone(),
                role: ChatCompletionMessageRole::Assistant,
                name: Some("LuminaBot".into()),
            },
            false => ChatCompletionMessage {
                // mutate the tag to only include a-Z, 0-9, - and _
                content: message.content.clone(),
                role: ChatCompletionMessageRole::User,
                name: Some(message.author.name.replace(|c: char| !c.is_ascii_alphanumeric() && c != '-' && c != '_', "")),
            },
        });
    }

    chat_gpt_messages
}

// using OpenAI, summarise the previous messages and generate a summary
// of the user's request, to be used as the query for the pinecone database
async fn get_user_request_summary(mut msgs: Vec<ChatCompletionMessage>) -> String {
    msgs.push(ChatCompletionMessage {
        content: "Messages are in chronological order, with the last being the most recent
        Summarise the user's request and intent, to be used as the query for relevant information. Be specific and include 5 related keywords".into(),
        role: ChatCompletionMessageRole::System,
        name: None,
    });

    let chat_completion = openai::chat::ChatCompletion::builder(MODEL, msgs.clone())
        .create()
        .await
        .unwrap()
        .unwrap();

    println!("{:?} request summary: {}", msgs[msgs.len() - 2].name, chat_completion.choices[0].message.content);

    chat_completion.choices[0].message.content.clone()
}

async fn query_pinecone_with_query(query: &str) -> Vec<VectorQueryMatch> {
    let embedding = openai::embeddings::Embedding::create("text-embedding-ada-002", &query, "lumina-bot")
        .await
        .unwrap()
        .unwrap();

    let client = pinecone_lib::PineconeClient::new();

    let mut matches = client.vector_query(embedding.vec, 5)
        .await
        .unwrap()
        .matches;

    matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    matches
}

#[async_trait]
impl EventHandler for Handler {
    // A LuminaGPTBot which replies to messages from users, and loads the previous messages context
    // Uses ChatGPT from OpenAI
    //
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return; // ignore messages from bots
        }

        // ignore messages that don't mention the bot
        if !msg.mentions_user_id(BOT_USER_ID) {
            return;
        }

        let typing = ctx.http.start_typing(msg.channel_id.0).unwrap();

        // get the 20 previous messages from the channel
        let mut chat_gpt_messages = get_user_messages(&msg, &ctx).await;
        let user_summary = get_user_request_summary(chat_gpt_messages.clone()).await;
        let matches = query_pinecone_with_query(&user_summary).await;

        // read the "text" metadata field of all the results, and join them together
        let text = matches
            .clone()
            .into_iter()
            .map(|result| {
                format!("PAGE:{}\nSCORE: {}\n{}",
                    result.id,
                    result.score,
                    result.metadata
                        .get("text")
                        .unwrap()
                        .as_str()
                        .unwrap()
                )
            })
            .collect::<Vec<String>>()
            .join("\n---\n");

        println!("Documents used:\n- {}",
            matches
                .into_iter()
                .map(|result| result.id)
                .collect::<Vec<String>>()
                .join("\n- ")
        );

        chat_gpt_messages.push(ChatCompletionMessage {
            content: format!("You are LuminaBot. Answer questions, and help develop plans for the Lumina new city/government project. Context:\n---\n{}", text),
            role: ChatCompletionMessageRole::System,
            name: None,
        });

        // create the request
        let chat_completion = openai::chat::ChatCompletion::builder("gpt-3.5-turbo", chat_gpt_messages)
            .create()
            .await
            .unwrap()
            .unwrap();

        // reply to the message
        msg.reply(ctx.http.clone(), chat_completion.choices[0].message.content.clone())
            .await
            .unwrap();

        let _ = typing.stop();
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    set_key(env::var("OPENAI_KEY").unwrap());

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MEMBERS;

    let mut client = serenity::Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    let http = client.cache_and_http.http.clone();

    // let channel = http.get_channel(1059758953921265705).await.unwrap().guild().unwrap();

    // channel.send_message(http.clone(), |m| m
    //     .content("Become a citizen today!")
    //     .components(|c| c
    //         // add a button
    //         .create_action_row(|r| r
    //             .create_button(|b| b
    //                 .label("Sign up")
    //                 .style(ButtonStyle::Link)
    //                 .url("https://lumina.earth/")
    //                 // .custom_id("citizen")
    //             )
    //         )
    //     )
    // ).await.unwrap();

    tokio::spawn(async move {
        start_running(http).await.unwrap();
    });

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {

    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[ignore]
#[tokio::test]
// We're going to upload all the files of the docs folder to the pinecone
// database, using OpenAI's API to get the embeddings for each file.
async fn upload_test_files() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();

    let pinecone = pinecone_lib::PineconeClient::new();
    // delete all the vectors
    pinecone.delete_all().await?;

    set_key(env::var("OPENAI_KEY").unwrap());

    let mut files = vec![];

    let mut paths = tokio::fs::read_dir("docs").await.unwrap();

    while let Ok(Some(path)) = paths.next_entry().await {
        let file_contents = get_file_contents(&path.path()).await;
        files.push((path.file_name(), file_contents));
    }

    // get embeddings for each file
    let res = openai::embeddings::Embeddings::create(
        "text-embedding-ada-002",
        files.iter().map(|(_, file)| file.as_str()).collect(),
        "lumina-bot",
    ).await.unwrap().unwrap();

    let mut vector_upserts = vec![];
    // iterate through the embeddings and files
    for ((name, file), embedding) in files.iter().zip(res.data.into_iter()) {
        // create a vector upsert
        let vector_upsert = pinecone_lib::VectorUpsert::new(
            name.to_string_lossy().into(),
            embedding.vec,
            json!({
                "text": file,
            })
        );

        // push the vector upsert to the vector upserts
        vector_upserts.push(vector_upsert);
    }


    // upload the embeddings to pinecone
    let uploaded_count = pinecone.vector_upsert(
        vector_upserts,
    ).await?;

    println!("Uploaded {} files", uploaded_count);

    Ok(())
}

async fn get_file_contents(path: &Path) -> String {
    let mut file = File::open(path).await.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();
    contents
}