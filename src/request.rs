#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub fn new(url: String) -> Request {
        Request {
            url
        }
    }
}