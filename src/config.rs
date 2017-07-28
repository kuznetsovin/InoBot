extern crate toml;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    pub inoreader_token: String,
    telegram_token: String
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

    // сделано для примера получения доступа к private полю структуры
    pub fn get_telegram_token(&self) -> &String {
        &self.telegram_token
    }
}