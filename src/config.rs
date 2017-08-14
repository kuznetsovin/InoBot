extern crate toml;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    pub inoreader_appkey: String,
    pub inoreader_appid: String,
    pub inoreader_token: String,
    pub inoreader_endpoint: String,
    telegram_token: String,
    telegram_endpoint: String,
    pub db_path: String,
    pub timeout: u64,
}

impl Config {
    pub fn new(path: &str) -> Config {
        let file_name = Path::new(path);

        let mut file = File::open(&file_name).unwrap();

        let mut content = String::new();

        //читаем содержимое файла, если ошибка паникуем
        file.read_to_string(&mut content).unwrap();

        // производим маппинг настроек из файла в стуруктуру
        toml::from_str(&content.to_string()).unwrap()
    }

    pub fn get_telegram_bot_endpoint(&self) -> String {
        let mut main_ep = self.telegram_endpoint.to_owned();
        let bot_ep = format!("/bot{}", &self.telegram_token);

        main_ep.push_str(&bot_ep);
        main_ep
    }
}