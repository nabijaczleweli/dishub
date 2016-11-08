use chrono::{FixedOffset, DateTime};
use json::{self, JsonValue};


#[derive(Clone, Debug)]
pub enum EventPayload {
    Member { action: String, user: String, },
    Other(JsonValue),
}

#[derive(Clone, Debug)]
pub struct Event {
    pub created_at: DateTime<FixedOffset>,
    pub actor: String,
    pub repo: String,
    pub payload: EventPayload,
}


impl Event {
    pub fn parse(what: &str) -> Vec<Event> {
        json::parse(what)
            .unwrap()
            .members()
            .map(|j| {
                Event {
                    created_at: DateTime::parse_from_rfc3339(j["created_at"].as_str().unwrap()).unwrap(),
                    actor: j["actor"]["display_login"].as_str().unwrap().to_string(),
                    repo: j["repo"]["name"].as_str().unwrap().to_string(),
                    payload: EventPayload::from(j),
                }
            })
            .collect()
    }
}

impl EventPayload {
    pub fn from(ev: &JsonValue) -> EventPayload {
        match ev["type"].as_str().unwrap() {
            "MemberEvent" => {
                EventPayload::Member {
                    action: ev["payload"]["action"].as_str().unwrap().to_string(),
                    user: ev["payload"]["member"]["login"].as_str().unwrap().to_string(),
                }
            }
            _ => EventPayload::Other(ev.clone()),
        }
    }
}
