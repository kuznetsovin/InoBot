//extern crate serde_json;

//use self::serde_json::Value;
use curl::easy::Easy;
//use std::str::FromStr;
use config::Config;

pub struct TelegramBotClient {
    client: Easy,
    endpoint: String
}

impl TelegramBotClient {
    pub fn new(conf: &Config) -> TelegramBotClient {
        TelegramBotClient {
            client: Easy::new(),
            endpoint: conf.get_telegram_bot_endpoint()
        }
    }

    pub fn get_me(&mut self) -> String {
        self.get("/getMe").unwrap()
    }

    fn get(&mut self, endpoint: &str) -> Result<String, u32> {
        let mut ep = self.endpoint.to_owned();
        let mut dst = Vec::new();

        ep.push_str(endpoint);
        &self.client.url(&ep).unwrap();

        // вынесено в для освобождения заимствования после получения ответа
        {
            let mut transfer = &mut self.client.transfer();
            transfer.write_function(|data| {
                dst.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }

        let response_code = self.client.response_code().unwrap();
        match response_code {
            200 => Ok(String::from_utf8(dst).unwrap()),
            _ => Err(response_code),
        }
    }

}