# Urbit Webhook Funnel

This is a simple Rust application which funnels external webhook event data to an Urbit chat. This application is intended to be run on a server (easiest setup is via running on the same server that the Urbit ship which will be sending the messages) and will automatically handle all authentication/communicating with the ship itself.

## Setup

Ensure that you have the [latest version of Rust installed](https://rustup.rs/) and the `libssl-dev` package on Ubuntu (aka `openssl-devel` on Fedora, and potentially slightly different on other distros).

1. Clone this repository and enter into the folder.

2. Run the setup script which will compile and prepare everything for you.

```sh
sh setup.sh
```

3. The Urbit Webhook Funnel application will be compiled, moved into the `deployed` folder, and two config files will be generated automatically for you.

4. Edit `ship_config.yaml` with your Urbit ship's ip/port/`+code`.

5. Edit `funnel_config.yaml` with the port that you wish for the webserver to run on, and the chat owner @p/chat name which you wish to funnel Webhook events to.

6. Run the application:

```sh
./urbit-webhook-funnel
```

7. The application will start and the webhook webserver will be on the port specified in the `funnel_config.yaml` configured to listen to webhook events on the `/webhook` endpoint. Thus, in your webhook emitting application/platform, the address you provide will look like:

```html
http://ip:port/webhook
```

Having the ability to choose which port a funnel uses enables for running multiple funnels at the same time on the same server, but each one pointed at a different chat and targeted by a different service. Whichever ports you choose. Please ensure that they are open or else the webhook events won't be able to reach your funnel.

You can visit `http://ip:port` while your funnel is running, and you will be greeted with a basic text page that provides you with instructive information about your given funnel (can be helpful when running multiple funnels):

![](https://i.imgur.com/3VUn0xd.png)

## Parsers

The Urbit Webhook Funnel attempts to parse any data that is sent to its webhook endpoint using all of the `EventParser`s that are currently implemented. If any one of them successfully parses the input json, then the event message will be posted using said predefined formatting (aka. make it look pretty). If none of the parsers succeed, then the body of the message is posted in its entirety (usually ugly).

Currently implemented parsers:

- GitLab

If you have a use case that requires a different parser, implementing one yourself is trivial. All you have to do is create a parser struct and implement the `EventParser` trait:

```rust
/// Trait for Webhook Event Parsers
pub trait EventParser {
    /// Takes in a pushed webhook event json as a string, attempts to
    /// parse said json, and returns a list of Urbit chat `Message`s to be submit
    /// to the chat as properly formatted/processed messages.
    /// Returns `None` if input json is not supported.
    fn parse_json(&self, json_string: &str) -> Option<Vec<Message>>;
}
```

If you happen to implement an alternative parser for a new use case, please PR to add it to this codebase so that other may also potentially take advantage of the work already done.

## Special Thanks

This application was possible thanks to the generous sponsorship of `~mister-todteg`.
