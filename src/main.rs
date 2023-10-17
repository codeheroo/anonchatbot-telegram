use teloxide::{
    prelude::*,
    
    types::{Recipient}};
#[tokio::main]
async fn main() {
    

    let bot = Bot::new("BOT_TOKEN HERE");

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(index) = msg.text().unwrap().find(' ') {
            println!("{}", &msg.text().unwrap()[0..index]);
            bot.send_message(Recipient::from(String::from(&msg.text().unwrap()[0..index])), String::from(msg.text().unwrap())).await?;
        }

        bot.send_message(msg.chat.id, String::from("Your message sent")).await?;
        Ok(())
    })
    .await;
}