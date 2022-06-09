# Tweety
A wrapper for twitter v2.0 api written for rust.

## An example
Create a new `tweety::Bot` struct and send a message with it.
```rs
use tweety::Bot;

fn main() {
    let bot = Bot::new(
        access_token: "twitter bot access token"
    )
    let response = bot.send_message("This is a test Tweet! :)").unwrap();
    println!("{:?}", response.data);
}
```