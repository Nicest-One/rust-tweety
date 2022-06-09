#![allow(unused)]

use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct Text {
    text: String
}

struct Bot {
    access_token: String
}


impl Bot {
    fn new(access_token: &str) -> Bot {
        Bot {access_token: access_token.to_string()}
    }
    fn send_message(self, text: &str) -> &str {
        let text = Text {text: text.to_string()};
        let serialized_text = serde_json::to_string(&text).unwrap();

        let result = Client::new()
            .post("https://api.twitter.com/2/tweets")
            .header("Content-type", "application/json")
            .header("Authorization", format!("Bearer {}", self.access_token))
            .body(serialized_text)
            .send();

        let returner = match result {
            Ok(response) => { 
                if response.status() == StatusCode::CREATED {
                    "message was sent"
                } else {
                    "message was not sent"
                }
            }
            Err(_err) => {
                "message was not sent"
            }
        };
        returner
    } 
}