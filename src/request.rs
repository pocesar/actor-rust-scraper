use reqwest::Client;
use crate::storage::request_text;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    // id: String,
    pub url: String,
    // unique_key: String,
    // method: String,
    // payload: String,
    // retry: bool,
    // retry_count: i32,
    // error_messages: Vec<String>,
    // headers: HashMap<String, String>,
    // user_data: HashMap<String, String>,
    // handled_at: String
}

impl Request {
    pub fn new(url: String) -> Self {
        Request {
            url
        }
    }

    pub async fn fetch(&self, client: &Client) -> String {
        request_text(&client, &self.url).await.unwrap_or_else(|err| {
            debugln!("{}", err);
            "".to_owned()
        })
    }
}

impl Clone for Request {
    fn clone(&self) -> Self {
        Request {
            url: self.url.clone()
        }
    }
}
