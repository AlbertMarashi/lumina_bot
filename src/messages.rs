use serenity::{http::Http, model::prelude::component::ButtonStyle};

use crate::broadcast::broadcast_message;

const LUMINA_UNIVERSITY_DISCORD_LINK: &str = "https://discord.gg/SG5cNn8qrG";
const LUMINA_DISCORD_LINK: &str = "https://discord.gg/r4vNcUKktT";
const FLAG_HOLDER: &str = "https://media.graphassets.com/3g8lDhSOSoizmWanaYAw";
const FLAG: &str = "https://media.graphassets.com/DqW6jRzpQGOW40PuPTo0";
const THUMBNAIL: &str = "https://i.imgur.com/J6cfDL5.png";
pub async fn send_lumina_univeristy_message(http: &Http) -> Result<(), anyhow::Error> {
    broadcast_message(&http, |m| m
        .content(format!("# Hello Lumina Citizen!
# We're excited to announce that Lumina University is now open for enrollment!

- Lumina University is a next generation education platform, powered by AI technology, personalised learning, and made fun through gamification.
- By enrolling into Lumina University is an excellent opportunity to further your personal and professional development.
- You'll become part of a vibrant community of learners who share your desire to shape the future of Lumina and can help you achieve your goals.
- Furthermore, by becoming a student of Lumina University, you're also **supporting Lumina's vision** of building a society where citizens decide how they want to live, work, and play.

The funds we raise will **help Lumina develop its infrastructure** and tranisition into a professional organisation.

> Don't miss this opportunity to be part of the future and make a real difference.

https://luminauniversity.earth/

"))
        .embed(|e|e
            .color(0x7446F6)
            .title("Lumina University")
            .description("Lumina University is Lumina's education system and will teach citizens valuable skills all while helping Lumina")
            .fields(vec![
                ("What is Lumina University?", "Lumina University is Lumina's education system and will teach citizens valuable skills all while helping Lumina", true),
                ("How do I join Lumina University?", "Joining Lumina University is easy, simply click the button below where you can enroll in Lumina University", true),
                ("Will there be more courses?", "Yes, we're currently working on more courses and will be releasing them soon", false),
                ("Lumina Discord", LUMINA_DISCORD_LINK, true),
                ("Lumina University Discord", LUMINA_UNIVERSITY_DISCORD_LINK, true)
            ])
            .image("https://media.graphassets.com/5iklItS7RZuge0HzKyqJ")
            .thumbnail(THUMBNAIL)
            .url("https://luminauniversity.earth/")
            .author(|a| a
                .name("Government of Lumina")
                .icon_url("https://i.imgur.com/J6cfDL5.png")
            )
        )
        .components(|c| c
            .create_action_row(|row| row
                .create_button(|button| button
                    .label("Join Lumina University")
                    .url(LUMINA_UNIVERSITY_DISCORD_LINK)
                    .style(ButtonStyle::Link)
                )
            )
        )
    ).await?;

    Ok(())
}

/// A message to let citizens know that Lumina is launching siteforge.io and that they can support helping Lumina gain funds to build a new city-state
/// by upvoting our launch on producthunt.com
pub async fn send_siteforge_launch_message(http: &Http) -> Result<(), anyhow::Error> {
    broadcast_message(&http, |m| m
//         .content(format!("
// ## We need your support
// This is the founder of Lumina, I'm excited to announce that we're launching SiteForge.io, a platform that allows anyone to plan websites and write website content.

// We're launching SiteForge.io on ProductHunt and we need your support to help us gain traction and get more people to know about Lumina.

// - Please register an account on producthunt.com
// - Upvote our launch via the link provided

// ## How will this help Lumina
// By gaining traction on producthunt.com, I'll be able to fund Lumina's development, enabling us to pay for
// - Software engineers
// - Marketing
// - Designers
// - Database & Application hosting

// This is the easiest way to help Lumina, and it only takes **2 minutes of your time**.

// If you find our product valuable, we'd also love to have you as a customer, if you have any feedback, please let us know.

// ## [Upvote SiteForge.io - VOTING ONLY OPEN FOR 24 HRS](https://www.producthunt.com/posts/siteforge)
// Help us get to the top of producthunt.com by upvoting our launch via the link below.
// > Finally, chat and discuss more on the Lumina Discord Server, we need more regular activity!"))
        .content("
## [Upvote SiteForge.io - VOTING ONLY OPEN FOR 24 HRS](https://www.producthunt.com/posts/siteforge)

Help us get to the top of producthunt.com by upvoting our launch via the link.

We're currently ranking 5th, we urgently need your support to get to the top.
        ")
        .embed(|e|e
            .color(0x7446F6)
            .title("SiteForge.io Product Hunt Launch")
            .description("SiteForge.io is a platform that allows anyone to plan websites and write website content.")
            .fields(vec![
                ("What is SiteForge.io?", "SiteForge.io is a platform that allows anyone to plan websites and write website content.", true),
                ("How do I support Lumina?", "Please register an account on producthunt.com and upvote our launch via the link provided", true),
                ("Will there be more features?", "Yes, we're currently working on more features and will be releasing them soon", false),
                ("Lumina Discord", LUMINA_DISCORD_LINK, true),
                ("SiteForge.io", "https://siteforge.io", true)
            ])
            .image("https://i.imgur.com/rozDVGN.png")
            .thumbnail("https://i.imgur.com/Q7tvnac.png")
            .url("https://www.producthunt.com/posts/siteforge")
            .author(|a| a
                .name("Government of Lumina")
                .icon_url("https://i.imgur.com/J6cfDL5.png")
            )
        )
        .components(|c| c
            .create_action_row(|row| row
                .create_button(|button| button
                    .label("Upvote SiteForge.io")
                    .url("https://www.producthunt.com/posts/siteforge")
                    .style(ButtonStyle::Link)
                )
                .create_button(|button| button
                    .label("Join Lumina Discord")
                    .url(LUMINA_DISCORD_LINK)
                    .style(ButtonStyle::Link)
                )
                .create_button(|button| button
                    .label("SiteForge.io Website")
                    .url("https://siteforge.io")
                    .style(ButtonStyle::Link)
                )
            )
        )
    ).await?;

    Ok(())
}



    // channel.send_message(&http, |m|
    //     m
    //         .content("Dear citizen, we invite you as a founder of Lumina to be involved in community discussions:  https://discord.gg/N3yUCEdm9u")
    //         .components(|c| c
    //             .create_action_row(|row| row
    //                 // .create_input_text(|input| input
    //                 //     .placeholder("Hello World!")
    //                 //     .label("Input")
    //                 //     .style(InputTextStyle::Short)
    //                 //     .custom_id("foo")
    //                 // )
    //                 .create_button(|button| button
    //                     .label("Join Discord Server")
    //                     .url("https://discord.gg/N3yUCEdm9u")
    //                     .style(ButtonStyle::Link)
    //                 )
    //             )
    //         )
    //         .add_embed(|e| e
    //             .thumbnail("https://i.imgur.com/J6cfDL5.png")
    //             .image("https://s3.us-west-2.amazonaws.com/secure.notion-static.com/cc2257b8-fbd8-4d15-a381-48f52cc19593/Untitled.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20221212%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20221212T044245Z&X-Amz-Expires=86400&X-Amz-Signature=a57b13897d76f12ba49947d00cb41aabb4c97ffdabc4e6aa43daca6580d51dfa&X-Amz-SignedHeaders=host&response-content-disposition=filename%3D%22Untitled.png%22&x-id=GetObject")
    //             .title("Dear Lumina Citizen")
    //             .description(format!("{}\n\n{}\n\n{}\n\n{}",
    //                 "Remember us? Lumina is still building a new city-state 🏗️",
    //                 "As a founder of Lumina, you're invited to help us build a new city-state from the ground up.",
    //                 "We encourage you to participate in the discussions and contribute your unique perspectives and experiences.",
    //                 "We rely on citizens like you to keep our movement going."
    //             ))
    //             .color(0x7446F6)
    //             .footer(|f| f
    //                 .text("Authorised by the Government of Lumina")
    //                 .icon_url("https://i.imgur.com/J6cfDL5.png")
    //             )
    //             .author(|a| a
    //                 .name("Government of Lumina")
    //                 .icon_url("https://i.imgur.com/J6cfDL5.png")
    //             )
    //             .fields([
    //                 ("Current projects", "Below you will find our ongoing projects", false),
    //                 ("Education System", "Light University will be Lumina's education system and will teach citizens valuable skills that help them get paid, and help our government", false),
    //                 ("Banking & Currency", "Lumina is creating it's own custom currency called Aura, and citizens will be able to earn and exchange it between currencies", false),
    //                 ("Direct Democracy", "The government is developing a direct democracy platform where citizens can vote, make polls, discuss ideas and more.", false),
    //                 ("Business management", "Our organisation management system will allow our citizens to create their own businesses and earn money.", false),
    //                 ("Join our server", "https://discord.gg/N3yUCEdm9u", false),
    //             ])
    //         )
    // ).await?;