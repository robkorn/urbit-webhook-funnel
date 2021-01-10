use crate::EventParser;
use json;
use json::JsonValue;

struct GitLabParser {}

impl EventParser for GitLabParser {
    /// Takes in a pushed webhook event json as a string, attempts to
    /// parse said json, and returns a list of human readable strings to be submit
    /// as messages to the chat which are formatted properly.
    /// Returns `None` if input json is not supported.
    fn parse_json(json_string: &str) -> Option<Vec<String>> {
        let j = json::parse(json_string).ok()?;

        match j["object_kind"].as_str()? {
            "push" => GitLabParser::parse_push_event(j),
            "tag_push" => GitLabParser::parse_tag_event(j),
            _ => None,
        }
    }
}

impl GitLabParser {
    /// Parses a push event from GitLab into a list of strings
    fn parse_push_event(j: JsonValue) -> Option<Vec<String>> {
        todo!()
    }

    /// Parses a tag event from GitLab into a list of strings
    fn parse_tag_event(j: JsonValue) -> Option<Vec<String>> {
        todo!()
    }

    /// Parses project information from any GitLab event. To be reused within
    /// the actual event parser methods.
    fn parse_project_from_json(j: JsonValue) -> Option<Vec<String>> {
        todo!()
    }
}
