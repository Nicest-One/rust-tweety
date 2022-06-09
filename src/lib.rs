// all "use" statements here
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use std::error::Error;


// Structure that has to do with the structure that creates a tweet
#[derive(Serialize, Deserialize, Debug)]
pub struct TweetText {
    pub text: String
}





// Main strucutures including data from tweet creation response
pub struct TweetCreateData {
    pub data: TweetIDandText,
    pub code: Status
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TweetJsonCreateData {
    pub data: TweetIDandText,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TweetIDandText {
    pub id: String,
    pub text: String
}



// Tweet creations status
pub enum Status {
    Forbidden,
    Created,
    Unauthorized,
    None
}





// Main strucutre
pub struct Bot {
    pub access_token: String
}

impl Bot {
    pub fn new(access_token: &str) -> Bot {
        Bot {access_token: access_token.to_string()}
    }
    pub fn send_message(self, text: &str) -> Result<TweetCreateData, Box<dyn Error>> {
        // serializes the text
        let text = TweetText {text: text.to_string()};
        let serialized_text = serde_json::to_string(&text).unwrap();
        
        // sends request to twitter
        let result = Client::new()
            .post("https://api.twitter.com/2/tweets")
            .header("Content-type", "application/json")
            .header("Authorization", format!("Bearer {}", self.access_token))
            .body(serialized_text)
            .send();

        
        // generates rust response
        let returner: TweetCreateData = match result {
            Ok(response) => { 
                if response.status() == StatusCode::CREATED {
                    let temp_response = response.text()?;
                    let json: TweetJsonCreateData = serde_json::from_str(&temp_response)?;
                    
                    TweetCreateData {data: json.data, code: Status::Created}
                } else if response.status() == StatusCode::FORBIDDEN {
                    let json = TweetIDandText {id: "None".to_string(), text: "Could not create Tweet due to wrong access token!".to_string()};
                    TweetCreateData {data: json, code: Status::Forbidden}
                } else if response.status() == StatusCode::UNAUTHORIZED {
                    let json = TweetIDandText {id: "None".to_string(), text: "Not allowed to create a Tweet with duplicate content.".to_string()};
                    TweetCreateData {data: json, code: Status::Unauthorized}
                } else {
                    let json = TweetIDandText {id: "None".to_string(), text: "Something unexpected happened!".to_string()};
                    TweetCreateData {data: json, code: Status::None}                    
                }
            }
            Err(_err) => {
                let json = TweetIDandText {id: "None".to_string(), text: "Something unexpected happened!".to_string()};
                TweetCreateData {data: json, code: Status::None}       
            }
        };

        // return struct
        Ok(returner)
        
    } 
}
