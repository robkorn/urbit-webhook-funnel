mod config;
mod gitlab;

use crate::config::*;
use std::str::from_utf8;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use urbit_http_api::{create_new_ship_config_file, ship_interface_from_local_config};
// use uuid::Uuid;
use crossbeam::channel::{unbounded, Receiver, Sender};
use sincere::App;

/// Trait for Webhook Event Parsers
pub trait EventParser {
    /// Takes in a pushed webhook event json as a string, attempts to
    /// parse said json, and returns a list of human readable strings to be submit
    /// as messages to the chat which are formatted properly.
    /// Returns `None` if input json is not supported.
    fn parse_json(json_string: &str) -> Option<Vec<String>>;
}

fn main() {
    // Creates a local funnel config file if needed
    create_new_funnel_config_file();
    // Creates a local ship config file if needed, and exits if first launch so that the user can edit the configs
    if let Some(_) = create_new_ship_config_file() {
        println!("Configuration files created. Please edit them and start the Urbit Webhook Funnel via ./urbit-webhook-funnel.");
        std::process::exit(0);
    }

    // Create a Rust channel to send messages from the webhook api thread to
    // the ship interface thread.
    let (webhook_tx, webhook_rx) = unbounded::<String>();

    thread::Builder::new()
        .name("Urbit Webhook Bot API Thread".to_string())
        .spawn(|| webserver_logic(webhook_tx))
        .ok();

    // print!("{}[2J", 27 as char);
    ship_interaction_logic(webhook_rx)
}

pub fn ship_interaction_logic(webhook_rx: Receiver<String>) {
    // Creates a `ShipInterace` from local config
    let ship_res = ship_interface_from_local_config();
    if let None = ship_res {
        println!("Failed to connect to Ship using information from local config.");
        std::process::exit(1);
    }
    let ship = ship_res.unwrap();
    // Creates a `Channel` with the Urbit Ship to communicate with it.
    let mut channel = ship.create_channel().unwrap();

    loop {
        if let Ok(response_json_string) = webhook_rx.try_recv() {
            let _poke_res = (&mut channel).poke("hood", "helm-hi", &response_json_string);
        }
        thread::sleep(Duration::new(1, 0));
    }
}

pub fn webserver_logic(webhook_tx: Sender<String>) {
    // Instantiate webserver struct
    let mut app = App::new();

    // Root GET
    app.get("/", |context| {
        context
            .response
            .from_text("The Urbit Webhook Bot Is Live And Running.")
            .unwrap();
    });

    // Webhook POST
    app.post("/webhook", move |context| {
        let res_json = from_utf8(context.request.body())
            .map(|t| json::parse(t))
            .unwrap()
            .unwrap();
        println!("Json: {}", res_json.dump());
        webhook_tx.send(res_json.pretty(0)).ok();

        context.response.from_text("").unwrap();
    });

    app.run("0.0.0.0:9000", 2).unwrap();
}
