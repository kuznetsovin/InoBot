extern crate serde_json;

use self::serde_json::Value;
use std::str::FromStr;
use curl::easy::Easy;
use config::Config;
use inoreader::News;

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

    //    pub fn get_me(&mut self) -> Value {
    //         self.get("/getMe").unwrap()
    //    }

    pub fn get_chart_ids(&mut self) -> Vec<u64> {
        // getUpdates получает все сообщения которые были отправлены боту на текущий момент
        let response = self.get("/getUpdates").unwrap();

        // из этих сообщений получем id чатов для рассылки
        let messages = response["result"].as_array().unwrap();

        let mut chart_ids = Vec::new();
        for m in messages {
            chart_ids.push(
                m["message"]["chat"]["id"].as_u64().unwrap()
            )
        }
        chart_ids
    }

    pub fn send_message(&mut self, news: News, chat_id: u64) {
        let msg_url = format!(
            "/sendMessage?chat_id={}&text={}&parse_mode=Markdown",
            chat_id,
            news.to_markdown(),
        );
        match self.get(&msg_url) {
            Ok(s) => println!("News success send"),
            Err(e) => println!("Can't send news"),
        };
    }

    fn get(&mut self, endpoint: &str) -> Result<Value, String> {
        let mut ep = self.endpoint.to_owned();
        let mut response = Vec::new();
        ep.push_str(endpoint);
        &self.client.url(&ep).unwrap();

        // вынесено в для освобождения заимствования после получения ответа
        {
            let mut transfer = &mut self.client.transfer();
            transfer.write_function(|data| {
                response.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }

        let response_code = self.client.response_code().unwrap();
        match response_code {
            200 => Ok(self.parse_response(response)),
            _ => Err(self.get_error_msg(response)),
        }
    }

    fn parse_response(&self, response: Vec<u8>) -> Value {
        let resp = String::from_utf8(response).unwrap();
        serde_json::from_str(&resp).unwrap_or_default()
    }

    fn get_error_msg(&self, response: Vec<u8>) -> String {
        let resp = &self.parse_response(response);
        String::from_str(resp["description"].as_str().unwrap_or_default()).unwrap_or_default()
    }
}

