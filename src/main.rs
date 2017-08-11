#[macro_use]
extern crate serde_derive;
extern crate curl;

pub mod config;
mod inoreader;
mod telegram;

fn main() {
    // получаем список аргументов командной строки
    let cmd_args: Vec<_> = std::env::args().collect();

    let cfg = config::Config::new(&cmd_args[1]);

    let mut inoreader_client = inoreader::InoReaderClient::new(&cfg);
    let mut telegram_bot = telegram::TelegramBotClient::new(&cfg);

    let unread_count = inoreader_client.get_unread_count();
    let news: Vec<inoreader::News> = inoreader_client.get_last_news(unread_count);

    let chats = telegram_bot.get_chart_ids();
    for c in chats {
        println!("id: {}", c);
    }

    for n in news {
        telegram_bot.send_message(n, 89731)
    }


}
