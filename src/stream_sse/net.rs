use futures_util::StreamExt;
use reqwest::get;
use reqwest_eventsource::{Event, EventSource};
use serde_json;

use crate::input::{Game, InitOptions};

async fn fetch_init_options(address: &String) -> Result<InitOptions, Box<dyn std::error::Error>> {
    let response = get(format!("{}/init-options", address)).await?;
    let init_options = response.json::<InitOptions>().await?;
    return Ok(init_options);
}

pub async fn bootstrap(address: String) {
    let mut events = EventSource::get(format!("{}/events", address));
    let mut current_init_options: Option<InitOptions> = None;
    while let Some(event) = events.next().await {
        match event {
            // what if the sse re-opens with different init_options ? we can't support for the moment - message parsers should ignore the header line
            Ok(Event::Open) => {
                if let Ok(init_options) = fetch_init_options(&address).await {
                    println!("{}\n", serde_json::to_string(&init_options).unwrap());
                    current_init_options = Some(init_options);
                }
            }
            Ok(Event::Message(message)) => {
                if current_init_options.is_some() {
                    if let Ok(game_state) = serde_json::from_str::<Game>(&message.data) {
                        println!("{}\n", serde_json::to_string(&game_state).unwrap());
                    }
                }
            }
            Err(_) => todo!(),
        }
    }
}
