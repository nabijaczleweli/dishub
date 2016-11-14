mod display;
mod urls;

use chrono::DateTime;
use dishub::ops::{EventPayload, Event};


static EVENTS: &'static str = include_str!("../../../../test-data/two_events.json");


#[test]
fn parse() {
    assert_eq!(Event::parse(EVENTS),
               vec![Event {
                        created_at: DateTime::parse_from_rfc2822("Thu, 10 Nov 2016 00:42:18 +0000").unwrap(),
                        actor: "liigo".to_string(),
                        repo: "nabijaczleweli/cargo-update".to_string(),
                        id: 4844096927,
                        payload: EventPayload::Issues {
                            action: "opened".to_string(),
                            number: 11,
                            title: "'unknown error occurred': It".to_string(),
                            body: r"```C:\Users\liigo>cargo install-update```".to_string(),
                            labels: vec![],
                        },
                    },
                    Event {
                        created_at: DateTime::parse_from_rfc2822("Thu, 10 Nov 2016 10:48:04 +0000").unwrap(),
                        actor: "nabijaczleweli".to_string(),
                        repo: "nabijaczleweli/cargo-update".to_string(),
                        id: 4846163121,
                        payload: EventPayload::IssueComment {
                            action: "created".to_string(),
                            issue: 12,
                            body: "Your `.cargo.toml`?".to_string(),
                            id: 259659314,
                        },
                    }]);
}
