use curl::easy::{Easy, List};
use config::Config;

pub struct InoReaderClient<'a> {
    client: Easy,
    app_id: &'a String,
    app_key: &'a String,
    app_token: &'a String,
    endpoint: &'a String
}

impl<'a> InoReaderClient<'a> {
    pub fn new(conf: &Config) -> InoReaderClient {
        let mut client = InoReaderClient {
            client: Easy::new(),
            app_id: &conf.inoreader_appid,
            app_token: &conf.inoreader_token,
            app_key: &conf.inoreader_appkey,
            endpoint: &conf.inoreader_endpoint
        };
        client.set_headers();
        client
    }

    pub fn get_user_info(&mut self) {
        let api_endpoint = self.get("/user-info").unwrap();
        println!("{}", api_endpoint);
    }

    pub fn get_subscribe_list(&mut self) {
        let api_endpoint = self.get("/subscription/list").unwrap();
        println!("{}", api_endpoint);
    }

    fn set_headers(&mut self) {
        let mut header_list = List::new();

        let mut app_id: String = "AppId: ".to_owned();
        app_id.push_str(&self.app_id);
        header_list.append(&app_id).unwrap();

        let mut app_key: String = "AppKey: ".to_owned();
        app_key.push_str(&self.app_key);
        header_list.append(&app_key).unwrap();

        let mut app_token: String = "Authorization: GoogleLogin auth=".to_owned();
        app_token.push_str(&self.app_token);
        header_list.append(&app_token).unwrap();

        &self.client.http_headers(header_list).unwrap();
    }

    fn get(&mut self, endpoint: &'a str) -> Result<String, u32> {
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