use crate::GitLabParser;
use urbit_http_api::Message;

const ALL_PARSERS: [&dyn EventParser; 1] = [&GitLabParser {}];

/// Trait for Webhook Event Parsers
pub trait EventParser {
    /// Takes in a pushed webhook event json as a string, attempts to
    /// parse said json, and returns a list of Urbit chat `Message`s to be submit
    /// to the chat as properly formatted/processed messages.
    /// Returns `None` if input json is not supported.
    fn parse_json(&self, json_string: &str) -> Option<Vec<Message>>;
}

// Attempt to parse JSON using every parser in `ALL_PARSERS`
pub fn parse_json_using_any_parser(json_string: &str) -> Option<Vec<Message>> {
    for parser in ALL_PARSERS.iter() {
        if let Some(message) = parser.parse_json(json_string) {
            return Some(message);
        }
    }
    None
}

// Attempt to parse JSON using the specified parser
// pub fn parse_json_using_specific_parser(json_string: &str, parser: &str) -> Option<Vec<Message>> {
//     match parser {
//         "gitlab" => GitLabParser {}.parse_json(json_string),
//         _ => None,
//     }
// }
