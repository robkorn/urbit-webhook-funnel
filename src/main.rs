mod config;
mod gitlab;
mod parser;

use config::*;
use gitlab::GitLabParser;
use parser::*;
use std::str::from_utf8;
use std::thread;
use std::time::Duration;
use urbit_http_api::chat::Message;
use urbit_http_api::{create_new_ship_config_file, ship_interface_from_local_config};
// use uuid::Uuid;
use crossbeam::channel::{unbounded, Receiver, Sender};
use sincere::App;

fn main() {
    // Creates a local funnel config file if needed
    create_new_funnel_config_file();
    // Creates a local ship config file if needed, and exits if first launch so that the user can edit the configs
    if let Some(_) = create_new_ship_config_file() {
        println!("Configuration files created. Please edit them and start the Urbit Webhook Funnel via ./urbit-webhook-funnel.");
        std::process::exit(0);
    }

    // Error checking
    if let None = ship_interface_from_local_config() {
        println!("Failed to connect to Ship using information from local config.");
        std::process::exit(1);
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

// Logic for thread that communicates with Urbit Ship
pub fn ship_interaction_logic(webhook_rx: Receiver<String>) {
    let ship = ship_interface_from_local_config().unwrap();
    let funnel_ship_name = funnel_chat_ship_from_local_config().unwrap();
    let funnel_chat_name = funnel_chat_name_from_local_config().unwrap();
    // Creates a `Channel` with the Urbit Ship to communicate with it.
    let mut channel = ship.create_channel().unwrap();

    loop {
        if let Ok(response_json_string) = webhook_rx.try_recv() {
            // Attempt to parse json using every implemented parser
            if let Some(messages) = parse_json_using_any_parser(&response_json_string) {
                for mess in messages {
                    let _mess_res = channel.chat().send_message(
                        &funnel_ship_name,
                        &funnel_chat_name,
                        &mess.into(),
                    );
                }
            // If failed to parse json using all parsers, send whole json
            } else {
                println!("Failed parsing webhook json using available parsers. Pasting full json to chat.");
                let _mess_res = channel.chat().send_message(
                    &funnel_ship_name,
                    &funnel_chat_name,
                    &Message::new().add_text(&response_json_string),
                );
            }
        }
        thread::sleep(Duration::new(1, 0));
    }
}

// Logic for Webhook Web Server
pub fn webserver_logic(webhook_tx: Sender<String>) {
    let ship = ship_interface_from_local_config().unwrap();
    let funnel_port = funnel_port_from_local_config().unwrap();
    let funnel_ship_name = funnel_chat_ship_from_local_config().unwrap();
    let funnel_chat_name = funnel_chat_name_from_local_config().unwrap();
    // Instantiate webserver struct
    let mut app = App::new();

    // Root GET
    app.get("/", move |context| {
        let message = format!(
            "Your Urbit Webhook Funnel Is Live And Running\n---------------------------------------------\nConnected Ship @p: ~{}\nConnected Ship URL: {}\nFunnel Webserver Port: {}\nFunneling To Chat: {}/{}",
            ship.ship_name, ship.url,funnel_port_from_local_config().unwrap(), funnel_ship_name, funnel_chat_name,
        );
        context.response.from_text(&message).unwrap();
    });

    // Webhook POST
    app.post("/webhook", move |context| {
        let res_json = from_utf8(context.request.body())
            .map(|t| json::parse(t))
            .unwrap()
            .unwrap();
        webhook_tx.send(res_json.pretty(0)).ok();

        context.response.from_text("").unwrap();
    });

    app.run(&format!("0.0.0.0:{}", funnel_port), 2).unwrap();
}
