pub mod config {

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Config {
        token: Option<String>,
        prefix: Option<String>,
    }

    impl Config {
        pub fn new(configfile: &str) -> Self {
            let text: String = std::fs::read_to_string(configfile).unwrap();
            match serde_json::from_str(&text) {
                Ok(c) => return c,
                _ => {
                    return Config {
                        token: None,
                        prefix: Some("!".to_string()),
                    }
                }
            };
        }

        //get config values with appropriate fallbacks and defaults
        pub fn get_token(self: &Self) -> String {
            match self.token.as_deref() {
                Some("ENV") | None  => std::env::var("DISCORD_TOKEN").expect("Unable to determine token!"),
                Some(token) => token.to_string(),
            }
        }

        pub fn get_prefix(self: &Self) -> String {
            match &self.prefix {
                Some(prefix) => prefix.to_string(),
                None => "!".to_string(),
            }
        }

        //set config values
        pub fn set_token(&mut self, token: &str) {
            self.token = Some(token.to_string());
        }

        pub fn set_prefix(&mut self, prefix: &str) {
            self.prefix = Some(prefix.to_string());
        }

    }
}
