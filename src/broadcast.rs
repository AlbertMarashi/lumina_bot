use serenity::{http::{Http, error::{ErrorResponse, DiscordJsonError}}, builder::CreateMessage, prelude::HttpError};


const SERVER_ID: u64 = 463886821051465728;

async fn get_users(http: &Http) -> Result<Vec<u64>, anyhow::Error> {

    let members = http.get_guild_members(SERVER_ID.into(), None, None).await?;

    let user_ids: Vec<u64> = vec![900184529623474237];
    let user_ids = members.iter()
        // we want to skip all the bots
        .filter(|m| !m.user.bot)
        // skip the user id
        .map(|m| m.user.id.0).collect::<Vec<u64>>();

    Ok(user_ids)
}

async fn send_message<'a, F>(http: &Http, user_id: u64, message: F) -> Result<(), anyhow::Error>
where
    for<'b> F: Fn(&'b mut CreateMessage<'a>) -> &'b mut CreateMessage<'a>,
{
    let user = http.get_user(user_id).await?;
    let dm = user.create_dm_channel(&http).await?;
    match dm.send_message(http,  &message).await {
        Ok(_) => println!("Sent message to {}", user.name),
        Err(e) => match e {
            serenity::Error::Http(err) => match *err {
                HttpError::UnsuccessfulRequest(ErrorResponse {
                    error: DiscordJsonError {
                        code: 50007,
                        ..
                    },
                    ..
                }) => println!("User {} has messages blocked", user.name),
                e => println!("Error sending message to {}: {:#?}", user.name, e),
            }
            _ => println!("Error sending message to {}: {}", user.name, e),
        }
    }
    Ok(())
}

pub async fn broadcast_message<'a, F>(http: &Http, message: F) -> Result<(), anyhow::Error>
where
    for<'b> F: Fn(&'b mut CreateMessage<'a>) -> &'b mut CreateMessage<'a>,
{
    let user_ids = get_users(http).await?;

    for user_id in user_ids {
        send_message(http, user_id, &message).await?;
    }

    Ok(())
}




    // get all the messages from this channel 1090198593631174746

//     let channel = http.get_channel(1090198593631174746).await?.guild().ok_or(anyhow::anyhow!("No guild"))?;

//     let mut messages = channel.messages(&http, |m| m.limit(200))
//         .await?;

//     messages.reverse();

//     // we then need to create a Vec<Vec<Message>> which contains all the messages
//     // every time the bot sees a message from user 900184529623474237, it will create a new Vec<Message>
//     // containing all the messages preceding that message

//     let mut messages_with_user: Vec<Vec<Message>> = Vec::new();

//     let mut i = 0;

//     for message in messages.iter() {
//         if message.author.id.0 == 900184529623474237 {
//             let mut msgs = Vec::new();

//             // push all the messages preceding this message and the message itself
//             for j in 0..i + 1 {
//                 msgs.push(messages[j].clone());
//             }

//             // allow a maximum of 10 messages, but remove any over this from the front
//             if msgs.len() > 10 {
//                 msgs.drain(0..msgs.len() - 10);
//             }

//             println!("{}", msgs.len());
//             messages_with_user.push(msgs);
//         }
//         i += 1;
//     }

//     #[derive(Serialize)]
//     struct TrainingPrompt {
//         prompt: String,
//         user: String,
//         completion: String,
//     }

//     impl TrainingPrompt {
//         fn new(prompt: String, user: String, completion: String) -> Self {
//             Self {
//                 prompt,
//                 user,
//                 completion,
//             }
//         }
//     }

//     // export the messages to a json file
//     let mut training_prompts: Vec<TrainingPrompt> = Vec::new();

//     for messages in messages_with_user.iter_mut() {
//         let mut prompt = String::new();

//        // prompt should be formatted like this:
//        // @<{user}>: \n
//        // {message} \n
//        // ----

//         let response = messages.pop().unwrap();

//         for message in messages.iter() {
//             prompt.push_str(&format!("@{}: \n", message.author.id));
//             prompt.push_str(&format!("{} \n", message.content));
//             prompt.push_str("---- \n");
//         }

//         training_prompts.push(TrainingPrompt::new(prompt, messages[messages.len() - 1].author.name.clone(), response.content));
//     }


//     let json = serde_json::to_string_pretty(&training_prompts).unwrap();

//     std::fs::write("messages.json", json).unwrap();
//     let guilds = http.get_guilds(None, None).await?;
//     for guild in guilds.iter() {
//         println!("{}", guild.name);
//     }
//     let guild = &guilds[0];

//     let members = http.get_guild_members(guild.id.into(), None, None).await?;

//     let user_ids: Vec<u64> = vec![900184529623474237];
//     let user_ids = members.iter()
//         // we want to skip all the bots
//         .filter(|m| !m.user.bot)
//         // skip all members, up until the user with the name "Doeke"
//         .skip_while(|m| m.user.name != "Chillaxitives")
//         // skip the user id
//         .map(|m| m.user.id.0).collect::<Vec<u64>>();

//     let channel = http.get_channel(964776525280985089).await?.guild().ok_or(anyhow::anyhow!("No guild"))?;


//     channel.send_message(http.clone(), |m| m
//         .flags(MessageFlags::EPHEMERAL)
//         .content("Hello world")
//     ).await?;

//     // for each user, get their dm and send a message
//     for user_id in user_ids {
//         let user = http.get_user(user_id).await?;
//         let dm = user.create_dm_channel(&http).await?;
//         match dm.send_message(&http, |m|
//             m
//                 .content("**Dear citizen**, we invite you as a founder of Lumina to become a community pioneer and get involved in discussions: Say \"Interested\" on our discord for more information: https://discord.gg/N3yUCEdm9u")
//                 .components(|c| c
//                     .create_action_row(|row| row
//                         // .create_input_text(|input| input
//                         //     .placeholder("Hello World!")
//                         //     .label("Input")
//                         //     .style(InputTextStyle::Short)
//                         //     .custom_id("foo")
//                         // )
//                         .create_button(|button| button
//                             .label("Join Discord Server")
//                             .url("https://discord.gg/N3yUCEdm9u")
//                             .style(ButtonStyle::Link)
//                         )
//                     )
//                 )
//                 .add_embed(|e| e
//                     .thumbnail("https://i.imgur.com/J6cfDL5.png")
//                     .image("https://s3.us-west-2.amazonaws.com/secure.notion-static.com/0750b784-696d-44b9-b044-9bacde4de949/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20221229%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20221229T100527Z&X-Amz-Expires=86400&X-Amz-Signature=d5b896f31153237c68ef3ebcab905505b48d406f08b7aff64949b44d0298614d&X-Amz-SignedHeaders=host&response-content-disposition=filename%3D%22Untitled.png%22&x-id=GetObject")
//                     .title("Looking for citizens to join our weekly meetings")
//                     .description("
// We invite you as a founder of Lumina to become a community pioneer and get involved in weekly discussions\n
// Say **\"Interested\"** on our discord for more information
//                     ")
//                     .color(0x7446F6)
//                     .footer(|f| f
//                         .text("Authorised by the Government of Lumina. This channel is not monitored")
//                         .icon_url("https://i.imgur.com/J6cfDL5.png")
//                     )
//                     .author(|a| a
//                         .name("Government of Lumina")
//                         .icon_url("https://i.imgur.com/J6cfDL5.png")
//                     )
//                     .fields([
//                         // ("Current projects", "Below you will find our ongoing projects", false),
//                         // ("Education System", "Light University will be Lumina's education system and will teach citizens valuable skills that help them get paid, and help our government", false),
//                         // ("Banking & Currency", "Lumina is creating it's own custom currency called Aura, and citizens will be able to earn and exchange it between currencies", false),
//                         // ("Direct Democracy", "The government is developing a direct democracy platform where citizens can vote, make polls, discuss ideas and more.", false),
//                         // ("Business management", "Our organisation management system will allow our citizens to create their own businesses and earn money.", false),
//                         // ("TikTok Following", "We now sit at over 12,000 followers on our TikTok account, and we're looking to grow it even more", false),
//                         ("Follow our twitter", "Help us gain credibility by following our twitter: https://twitter.com/LuminaGov", false),
//                         ("Join our server", "https://discord.gg/N3yUCEdm9u", false),
//                     ])
//                 )
//         ).await {
//             Ok(_) => println!("Sent message to {}", user.name),
//             Err(e) => match e {
//                 serenity::Error::Http(err) => match *err {
//                     HttpError::UnsuccessfulRequest(ErrorResponse {
//                         error: DiscordJsonError {
//                             code: 50007,
//                             ..
//                         },
//                         ..
//                     }) => println!("User {} has messages blocked", user.name),
//                     e => println!("Error sending message to {}: {:#?}", user.name, e),
//                 }
//                 _ => println!("Error sending message to {}: {}", user.name, e),
//             }
//         }

//         tokio::time::sleep(Duration::from_millis(10)).await;
//     }

//     channel.send_message(&http, |m|
//         m
//             .content("Dear citizen, we invite you as a founder of Lumina to be involved in community discussions:  https://discord.gg/N3yUCEdm9u")
//             .components(|c| c
//                 .create_action_row(|row| row
//                     // .create_input_text(|input| input
//                     //     .placeholder("Hello World!")
//                     //     .label("Input")
//                     //     .style(InputTextStyle::Short)
//                     //     .custom_id("foo")
//                     // )
//                     .create_button(|button| button
//                         .label("Join Discord Server")
//                         .url("https://discord.gg/N3yUCEdm9u")
//                         .style(ButtonStyle::Link)
//                     )
//                 )
//             )
//             .add_embed(|e| e
//                 .thumbnail("https://i.imgur.com/J6cfDL5.png")
//                 .image("https://s3.us-west-2.amazonaws.com/secure.notion-static.com/cc2257b8-fbd8-4d15-a381-48f52cc19593/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20221212%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20221212T044245Z&X-Amz-Expires=86400&X-Amz-Signature=a57b13897d76f12ba49947d00cb41aabb4c97ffdabc4e6aa43daca6580d51dfa&X-Amz-SignedHeaders=host&response-content-disposition=filename%3D%22Untitled.png%22&x-id=GetObject")
//                 .title("Dear Lumina Citizen")
//                 .description(format!("{}\n\n{}\n\n{}\n\n{}",
//                     "Remember us? Lumina is still building a new city-state 🏗️",
//                     "As a founder of Lumina, you're invited to help us build a new city-state from the ground up.",
//                     "We encourage you to participate in the discussions and contribute your unique perspectives and experiences.",
//                     "We rely on citizens like you to keep our movement going."
//                 ))
//                 .color(0x7446F6)
//                 .footer(|f| f
//                     .text("Authorised by the Government of Lumina")
//                     .icon_url("https://i.imgur.com/J6cfDL5.png")
//                 )
//                 .author(|a| a
//                     .name("Government of Lumina")
//                     .icon_url("https://i.imgur.com/J6cfDL5.png")
//                 )
//                 .fields([
//                     ("Current projects", "Below you will find our ongoing projects", false),
//                     ("Education System", "Light University will be Lumina's education system and will teach citizens valuable skills that help them get paid, and help our government", false),
//                     ("Banking & Currency", "Lumina is creating it's own custom currency called Aura, and citizens will be able to earn and exchange it between currencies", false),
//                     ("Direct Democracy", "The government is developing a direct democracy platform where citizens can vote, make polls, discuss ideas and more.", false),
//                     ("Business management", "Our organisation management system will allow our citizens to create their own businesses and earn money.", false),
//                     ("Join our server", "https://discord.gg/N3yUCEdm9u", false),
//                 ])
//             )
//     ).await?;

//     let user = http.get_user(900184529623474237).await?;
//     user.direct_message(&http, |m| {
//         m
//             .content("Hello, world!")
//             .embed(|e| e
//                 .title("This is an embed")
//                 .description("This is a description"))
//     }).await?;
//     println!("{}", user);


//         async fn message(&self, ctx: Context, new_msg: Message) {
//         let moderator_channel_id = 768961135608987678;

//         // skip channels that aren't private DMs
//         match new_msg.channel(ctx.http.clone()).await {
//             Ok(Channel::Private(channel)) => channel,
//             _ => {},
//         };

//         // if the message is not from a bot, and the message is not from the bot itself, send it to the moderator channel
//         if !new_msg.author.bot {
//             let moderator_channel = ChannelId::from(moderator_channel_id).to_channel(&ctx.http).await.unwrap().guild().unwrap();

//             let message = moderator_channel
//                 .send_message(&ctx.http, |m| {
//                     m.content(format!("Bot DM from **{}**:\n\n{}", new_msg.author.name, new_msg.content))
//                 })
//                 .await;

//             match message {
//                 Ok(_) => println!("Message sent to moderator channel"),
//                 Err(e) => println!("Error sending message to moderator channel: {}", e),
//             }
//         }
//     }