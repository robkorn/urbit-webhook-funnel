# Urbit Webhook Funnel

This is a simple Rust application which funnels external webhook event data to an Urbit chat. This application is intended to be run on a server (easiest setup is via running on the same server that the Urbit ship which will be sending the messages) and will automatically handle all authentication/communicating with the ship itself.

## Setup

Ensure that you have the [latest version of Rust installed](https://rustup.rs/) and the `libssl-dev` package on Ubuntu (aka `openssl-devel` on Fedora, and potentially slightly different on other distros).

1. Clone this repository and enter into the folder.

2. Run the setup script which will compile and prepare everything for you.

```sh
sh setup.rs
```

3. The Urbit Webhook Funnel application will be compiled, moved into the `deployed` folder, and two config files will be generated automatically for you.

4. Edit `ship_config.yaml` with your Urbit ships ip/port/`+code`.

5. Edit `funnel_config` with the path (chat owner @p + chat name) to the chat which you wish to funnel Webhook events to.

6. Run the application:

```sh
./urbit-webhook-funnel
```

7. The application will start and the webhook webserver will be on the port specified in the `funnel_config.yaml` configured to listen to webhook events on the `/webhook` endpoint. Thus in your webhook emitting application/platform, the address you provide will look like:

```html
http://ip:port/webhook
```

Ensure that the port you chose is open. You can visit `http://ip:port` while the Urbit Webhook Funnel is running, and you will be greeted by boilerplate text that verifies your port is open and the application is working.

## Parsers

The Urbit Webhook Funnel attempts to parse any data that is sent to its webhook endpoint using all of the `EventParser`s that are currently implemented. If any one of them successfully parses the input json, then the event message will be posted using said predefined formatting (aka. make it look pretty). If none of the parsers succeed, then the body of the message is posted in it's entirety (usually ugly).

Currently implemented parsers:

- GitLab

If you have a use case that requires a different parser, implementing one yourself is trivial. All you have to do is create a parser struct and implement the `EventParser` trait:

```rust
/// Trait for Webhook Event Parsers
pub trait EventParser {
    /// Takes in a pushed webhook event json as a string, attempts to
    /// parse said json, and returns a list of human readable strings to be submit
    /// as messages to the chat which are formatted properly.
    /// Returns `None` if input json is not supported.
    fn parse_json(&self, json_string: &str) -> Option<Vec<String>>;
}
```

If you happen to implement an alternative parser for a new use case, please PR to add it to this codebase so that other may also potentially take advantage of the work already done.

## Special Thanks

This application was possible thanks to the generous sponsorship of `~mister-todteg`.
