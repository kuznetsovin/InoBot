#[macro_use]
extern crate serde_derive;
extern crate curl;

pub mod config;
mod inoreader;

fn main() {
    // получаем список аргументов командной строки
    let cmd_args: Vec<_> = std::env::args().collect();

    let cfg = config::Config::new(&cmd_args[1]);

    let mut inoreader_client = inoreader::InoReaderClient::new(&cfg);

    inoreader_client.get_user_info();
    inoreader_client.get_subscribe_list();
}
