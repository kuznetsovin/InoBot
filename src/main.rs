#[macro_use]
extern crate serde_derive;
extern crate curl;
extern crate rusqlite;

pub mod config;
mod inoreader;
mod telegram;
mod store;

use std::thread;
use std::process;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    // получаем список аргументов командной строки
    let cmd_args: Vec<_> = std::env::args().collect();

    if cmd_args.len() < 2 {
        println!("Illegal parameter use: inobot <config_path>");
        process::exit(1);
    }
    let cfg = config::Config::new(&cmd_args[1]);
    let store = store::Store::connect(&cfg);

    let mut inoreader_client = inoreader::InoReaderClient::new(&cfg);
    let mut telegram_bot = telegram::TelegramBotClient::new(&cfg);

    loop {
        let new_chats = telegram_bot.get_chart_ids();
        let mut saved_charts = store.get_chart_ids();
        for c in new_chats {
            if !saved_charts.contains(&c) {
                store.add_chart(c);
                saved_charts.push(c);
            }
        }

        let unread_count = inoreader_client.get_unread_count();
        let news: Vec<inoreader::News> = inoreader_client.get_last_news(unread_count);

        for c in saved_charts {
            let sending_news = news.clone();
            let mut bot = telegram::TelegramBotClient::new(&cfg);
            let _ = thread::spawn(move || {
                for n in sending_news {
                    bot.send_message(n, c);
                }
            }).join();
        }

        // ждем timeout в секундах для следующего обращения
        sleep(Duration::from_secs(cfg.timeout));
    }
}
