use curl::easy::{Easy, List};
use config::Config;

pub struct InoReaderClient<'a> {
    client: Easy,
    app_id: &'a String,
    app_key: &'a String,
    app_token: &'a String,
}

impl<'a> InoReaderClient<'a> {
    pub fn new(conf: &Config) -> InoReaderClient {
        let mut client = InoReaderClient {
            client: Easy::new(),
            app_id: &conf.inoreader_appid,
            app_token: &conf.inoreader_token,
            app_key: &conf.inoreader_appkey,
        };
        client.set_headers();
        client
    }

    pub fn get_user_info(&mut self) {
        let api_endpoint = "https://www.inoreader.com/reader/api/0/user-info";

        &self.client.url(api_endpoint).unwrap();
        &self.client.perform().unwrap();

        println!("{}", &self.client.response_code().unwrap());
    }

    pub fn get_subscribe_list(&mut self) {
        let api_endpoint = "https://www.inoreader.com/reader/api/0/subscription/list";
        &self.client.url(api_endpoint).unwrap();
        &self.client.perform().unwrap();

        println!("{}", &self.client.response_code().unwrap());
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
}