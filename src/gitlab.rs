use crate::EventParser;
use json::JsonValue;
use urbit_http_api::Message;

pub struct GitLabParser {}

impl EventParser for GitLabParser {
    /// Takes in a pushed webhook event json as a string, attempts to
    /// parse said json, and returns a list of human readable strings to be submit
    /// as messages to the chat which are formatted properly.
    /// Returns `None` if input json is not supported.
    fn parse_json(&self, json_string: &str) -> Option<Vec<Message>> {
        let j = json::parse(json_string).ok()?;

        match j["object_kind"].as_str()? {
            "push" => Self::parse_push_event(j),
            "tag_push" => Self::parse_tag_event(j),
            "issue" => Self::parse_issue_event(j),
            "note" => Self::parse_comment_event(j),
            "merge_request" => Self::parse_merge_request_event(j),
            "wiki_page" => Self::parse_wiki_page_event(j),
            "build" => Self::parse_job_event(j),
            "deployment" => Self::parse_deployment_event(j),
            "feature_flag" => Self::parse_feature_flag_event(j),
            "release" => Self::parse_release_event(j),
            _ => None,
        }
    }
}

impl GitLabParser {
    /// Parses a push event from GitLab into a list of strings
    fn parse_push_event(j: JsonValue) -> Option<Vec<Message>> {
        let mut messages = Self::parse_default_message_header(j.clone())?;

        let commits = j["commits"].members();
        for commit in commits {
            let string = format!("    +  {}: {}", commit["id"], commit["title"]);
            messages.push(Message::new().add_text(&string));
        }
        messages.push(Message::new().add_text("=========="));
        Some(messages)
    }

    /// Parses a tag event from GitLab into a list of strings
    fn parse_tag_event(j: JsonValue) -> Option<Vec<Message>> {
        let mut messages = Self::parse_default_message_header(j)?;

        messages.push(Message::new().add_text("=========="));
        Some(messages)
    }

    /// Parses an issue event from GitLab into a list of strings
    fn parse_issue_event(j: JsonValue) -> Option<Vec<Message>> {
        let mut messages = Self::parse_default_message_header(j.clone())?;

        let issue_title = j["object_attributes"]["title"].clone();
        messages.push(Message::new().add_text(&format!("Title: {}", issue_title)));

        messages.push(Message::new().add_text("=========="));
        Some(messages)
    }

    /// Parses a comment event from GitLab into a list of strings
    fn parse_comment_event(j: JsonValue) -> Option<Vec<Message>> {
        let mut messages = Self::parse_default_message_header(j)?;

        messages.push(Message::new().add_text("=========="));
        Some(messages)
    }

    /// Parses a merge request event from GitLab into a list of strings
    fn parse_merge_request_event(j: JsonValue) -> Option<Vec<Message>> {
        let mut messages = Self::parse_default_message_header(j)?;

        messages.push(Message::new().add_text("=========="));
        Some(messages)
    }

    /// Parses a wiki page event from GitLab into a list of strings
    fn parse_wiki_page_event(j: JsonValue) -> Option<Vec<Message>> {
        let mut messages = Self::parse_default_message_header(j)?;

        messages.push(Message::new().add_text("=========="));
        Some(messages)
    }

    /// Parses a job event from GitLab into a list of strings
    fn parse_job_event(j: JsonValue) -> Option<Vec<Message>> {
        let mut messages = Self::parse_default_message_header(j)?;

        messages.push(Message::new().add_text("=========="));
        Some(messages)
    }

    /// Parses a deployment event from GitLab into a list of strings
    fn parse_deployment_event(j: JsonValue) -> Option<Vec<Message>> {
        let mut messages = Self::parse_default_message_header(j)?;

        messages.push(Message::new().add_text("=========="));
        Some(messages)
    }

    /// Parses a feature flag event from GitLab into a list of strings
    fn parse_feature_flag_event(j: JsonValue) -> Option<Vec<Message>> {
        let mut messages = Self::parse_default_message_header(j)?;

        messages.push(Message::new().add_text("=========="));
        Some(messages)
    }

    /// Parses a feature flag event from GitLab into a list of strings
    fn parse_release_event(j: JsonValue) -> Option<Vec<Message>> {
        let mut messages = Self::parse_default_message_header(j)?;

        messages.push(Message::new().add_text("=========="));
        Some(messages)
    }

    /// Creates a message header by parsing the json
    fn parse_default_message_header(j: JsonValue) -> Option<Vec<Message>> {
        let avatar = Self::parse_avatar(j.clone())?;
        let username = Self::parse_username(j.clone())?;
        let event_type = j["object_kind"].clone();
        let url = j["project"]["web_url"].clone().to_string();

        let mut messages = vec![];
        messages.push(Message::new().add_url(&avatar));
        messages.push(Message::new().add_text(&format!("{} -- {}", username, event_type)));
        messages.push(Message::new().add_url(&url));

        Some(messages)
    }

    /// Attempts to parse the username
    fn parse_username(j: JsonValue) -> Option<String> {
        let username = j["user_username"].clone();
        if !username.is_null() {
            return Some(username.as_str()?.to_string());
        }
        let username = j["user"]["username"].clone();
        if !username.is_null() {
            return Some(username.as_str()?.to_string());
        }
        println!("Failed to parse username.");
        None
    }

    /// Attempts to parse project avatar, else parses user avatar
    fn parse_avatar(j: JsonValue) -> Option<String> {
        let proj_avatar = j["project"]["avatar_url"].clone();
        if !proj_avatar.is_null() {
            return Some(proj_avatar.as_str()?.to_string());
        }
        let user_avatar = j["user_avatar"].clone();
        if !user_avatar.is_null() {
            return Some(user_avatar.as_str()?.to_string());
        }
        println!("Failed to parse project & user avatars.");
        None
    }
}
