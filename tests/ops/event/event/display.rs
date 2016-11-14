use json;
use chrono::DateTime;
use dishub::ops::{EventPayload, Event};


static COMMIT_COMMENT: &'static str = include_str!("../../../../test-data/commit_comment_payload.json");
static CREATE: &'static str = include_str!("../../../../test-data/create_payload.json");
static DELETE: &'static str = include_str!("../../../../test-data/delete_payload.json");
static FORK: &'static str = include_str!("../../../../test-data/fork_payload.json");
static GOLLUM: &'static str = include_str!("../../../../test-data/gollum_payload.json");
static ISSUE_COMMENT: &'static str = include_str!("../../../../test-data/issue_comment_payload.json");
static ISSUES: &'static str = include_str!("../../../../test-data/issues_payload.json");
static MEMBER: &'static str = include_str!("../../../../test-data/member_payload.json");
static PUBLIC: &'static str = include_str!("../../../../test-data/public_payload.json");
static PULL_REQUEST: &'static str = include_str!("../../../../test-data/pull_request_payload.json");
static PULL_REQUEST_REVIEW: &'static str = include_str!("../../../../test-data/pull_request_review_payload.json");
static PULL_REQUEST_REVIEW_COMMENT: &'static str = include_str!("../../../../test-data/pull_request_review_comment_payload.json");
static PUSH: &'static str = include_str!("../../../../test-data/push_payload.json");
static RELEASE: &'static str = include_str!("../../../../test-data/release_payload.json");
static WATCH: &'static str = include_str!("../../../../test-data/watch_payload.json");
static OTHER_FORK_APPLY: &'static str = include_str!("../../../../test-data/other_fork_apply_payload.json");


#[test]
fn commit_comment() {
    assert_eq!(&format!("{}",
                        Event { payload: EventPayload::from("CommitCommentEvent", &json::parse(COMMIT_COMMENT).unwrap()), ..base() }),
               "10.11.2016 08:42:18 AM: liigo commented on 1ce17f668485fd6741895c3caba1f2ea77ab0e6c in nabijaczleweli/cargo-update");
}

#[test]
fn create() {
    assert_eq!(&format!("{}",
                        Event { payload: EventPayload::from("CreateEvent", &json::parse(CREATE).unwrap()), ..base() }),
               "10.11.2016 08:42:18 AM: liigo created tag v0.5.0");
}

#[test]
fn delete() {
    assert_eq!(&format!("{}",
                        Event { payload: EventPayload::from("DeleteEvent", &json::parse(DELETE).unwrap()), ..base() }),
               "10.11.2016 08:42:18 AM: liigo deleted tag v0.2.0");
}

#[test]
fn fork() {
    assert_eq!(&format!("{}", Event { payload: EventPayload::from("ForkEvent", &json::parse(FORK).unwrap()), ..base() }),
               "10.11.2016 08:42:18 AM: liigo forked nabijaczleweli/cargo-update to nabijaczleweli/clap-rs");
}

#[test]
fn gollum() {
    assert_eq!(&format!("{}",
                        Event { payload: EventPayload::from("GollumEvent", &json::parse(GOLLUM).unwrap()), ..base() }),
               "10.11.2016 08:42:18 AM: liigo changed wiki on nabijaczleweli/cargo-update:\n\
                \x20\x20edited \"Packages\"\n\
                \x20\x20edited \"Packages\"");
}

#[test]
fn issue_comment() {
    assert_eq!(&format!("{}",
                        Event { payload: EventPayload::from("IssueCommentEvent", &json::parse(ISSUE_COMMENT).unwrap()), ..base() }),
               "10.11.2016 08:42:18 AM: liigo created comment to #12 on nabijaczleweli/cargo-update");
}

#[test]
fn issues() {
    assert_eq!(&format!("{}",
                        Event { payload: EventPayload::from("IssuesEvent", &json::parse(ISSUES).unwrap()), ..base() }),
               "10.11.2016 08:42:18 AM: liigo opened #11 on nabijaczleweli/cargo-update: \"'unknown error occurred': It\"");
}

#[test]
fn member() {
    assert_eq!(&format!("{}",
                        Event { payload: EventPayload::from("MemberEvent", &json::parse(MEMBER).unwrap()), ..base() }),
               "10.11.2016 08:42:18 AM: liigo added Enet4 to nabijaczleweli/cargo-update");
}

#[test]
fn public() {
    assert_eq!(&format!("{}",
                        Event { payload: EventPayload::from("PublicEvent", &json::parse(PUBLIC).unwrap()), ..base() }),
               "10.11.2016 08:42:18 AM: liigo made nabijaczleweli/cargo-update public");
}

#[test]
fn pull_request() {
    assert_eq!(&format!("{}",
                        Event { payload: EventPayload::from("PullRequestEvent", &json::parse(PULL_REQUEST).unwrap()), ..base() }),
               "10.11.2016 08:42:18 AM: liigo merged #138 on nabijaczleweli/cargo-update: \"bumping version.h to 1.7.0\"");
}

#[test]
fn pull_request_review() {
    assert_eq!(&format!("{}",
                        Event { payload: EventPayload::from("PullRequestReviewEvent", &json::parse(PULL_REQUEST_REVIEW).unwrap()), ..base() }),
               "10.11.2016 08:42:18 AM: liigo submitted as approved #210 on nabijaczleweli/cargo-update");
}

#[test]
fn pull_request_review_comment() {
    assert_eq!(&format!("{}",
                        Event { payload: EventPayload::from("PullRequestReviewCommentEvent", &json::parse(PULL_REQUEST_REVIEW_COMMENT).unwrap()), ..base() }),
               "10.11.2016 08:42:18 AM: liigo created comment to #210 on nabijaczleweli/cargo-update");
}

#[test]
fn push() {
    assert_eq!(&format!("{}", Event { payload: EventPayload::from("PushEvent", &json::parse(PUSH).unwrap()), ..base() }),
               "10.11.2016 08:42:18 AM: liigo pushed 1 commit to doc in nabijaczleweli/cargo-update");
}

#[test]
fn release() {
    assert_eq!(&format!("{}",
                        Event { payload: EventPayload::from("ReleaseEvent", &json::parse(RELEASE).unwrap()), ..base() }),
               "10.11.2016 08:42:18 AM: liigo published v0.5.0 from master");
}

#[test]
fn watch() {
    assert_eq!(&format!("{}",
                        Event { payload: EventPayload::from("WatchEvent", &json::parse(WATCH).unwrap()), ..base() }),
               "10.11.2016 08:42:18 AM: liigo starred nabijaczleweli/cargo-update");
}

#[test]
fn other() {
    assert_eq!(&format!("{}",
                        Event { payload: EventPayload::from("ForkApplyEvent", &json::parse(OTHER_FORK_APPLY).unwrap()), ..base() }),
               "10.11.2016 08:42:18 AM: liigo invoked an unsupported event on nabijaczleweli/cargo-update: ForkApplyEvent");
}


fn base() -> Event {
    Event {
        created_at: DateTime::parse_from_rfc2822("Thu, 10 Nov 2016 08:42:18 +0000").unwrap(),
        actor: "liigo".to_string(),
        repo: "nabijaczleweli/cargo-update".to_string(),
        id: 4844096927,
        payload: EventPayload::Public,
    }
}
