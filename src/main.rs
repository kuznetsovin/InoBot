#[macro_use]
extern crate serde_derive;

mod config;

fn main() {
    // получаем список аргументов командной строки
    let cmd_args: Vec<_> = std::env::args().collect();

    let cfg = config::Config::new(&cmd_args[1]);

    println!("\ninoreader: {}\ntelegram: {}", cfg.inoreader_token, cfg.get_telegram_token())
}
