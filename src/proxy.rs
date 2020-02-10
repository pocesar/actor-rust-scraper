use crate::input::ProxySettings;
use std::env;
use std::convert::{ TryFrom };

#[derive(Debug)]
pub struct Proxy {
    pub base_url: String,
    pub username: String,
    pub password: String
}

impl Proxy {
    fn new(username: String, password: String) -> Proxy {
        Proxy {
            base_url: "http://@proxy.apify.com:8000".to_owned(),
            username,
            password
        }
    }
}

impl Clone for Proxy {
    fn clone(&self) -> Self {
        Proxy {
            base_url: self.base_url.to_string(),
            password: self.password.to_string(),
            username: self.username.to_string()
        }
    }
}

impl TryFrom<Option<ProxySettings>> for Proxy {
    type Error = ();
    fn try_from(settings: Option<ProxySettings>) -> Result<Self, Self::Error> {
        if let Some(settings) = settings {
            if settings.useApifyProxy {
                let password = env::var("APIFY_PROXY_PASSWORD")
                    .expect("Missing APIFY_PROXY_PASSWORD environment variable. This is required to use Apify proxy!");

                let username = settings.apifyProxyGroups
                    .map(|groups| format!("groups-{}", groups.join("+")))
                    .unwrap_or_else(|| "auto".to_owned());

                return Ok(Proxy::new(username, password))
            }
        }

        Err(())
    }
}
