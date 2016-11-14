use chrono::DateTime;
use dishub::ops::{GollumPayload, EventPayload, Event};


#[test]
fn commit_comment() {
    assert_eq!(Event {
                       payload: EventPayload::CommitComment {
                           content: "CommitComment content".to_string(),
                           commit_id: "62476f13306db1cfade222d41bcdcb51".to_string(),
                           id: 1234,
                       },
                       ..base()
                   }
                   .urls(),
               vec!["https://github.com/nabijaczleweli/cargo-update/commit/62476f13306db1cfade222d41bcdcb51#commitcomment-1234".to_string()]);
}

#[test]
fn create() {
    assert_eq!(Event {
                       payload: EventPayload::Create {
                           ref_type: "tag".to_string(),
                           ref_name: Some("man".to_string()),
                           master_branch: "master".to_string(),
                           repo_description: "A cargo subcommand for checking and applying updates to installed executables".to_string(),
                       },
                       ..base()
                   }
                   .urls(),
               vec!["https://github.com/nabijaczleweli/cargo-update/compare/man".to_string()]);
}

#[test]
fn create_repo() {
    assert_eq!(Event {
                       payload: EventPayload::Create {
                           ref_type: "repository".to_string(),
                           ref_name: None,
                           master_branch: "master".to_string(),
                           repo_description: "A cargo subcommand for checking and applying updates to installed executables".to_string(),
                       },
                       ..base()
                   }
                   .urls(),
               vec!["https://github.com/nabijaczleweli/cargo-update".to_string()]);
}

#[test]
fn delete() {
    assert!(Event {
            payload: EventPayload::Delete {
                ref_type: "tag".to_string(),
                ref_name: "v0.1.0".to_string(),
            },
            ..base()
        }
        .urls()
        .is_empty());
}

#[test]
fn fork() {
    assert_eq!(Event { payload: EventPayload::Fork { new_slug: "liigo/cargo-update".to_string() }, ..base() }.urls(),
               vec!["https://github.com/liigo/cargo-update".to_string()]);
}

#[test]
fn gollum() {
    assert_eq!(Event {
                       payload: EventPayload::Gollum {
                           pages: vec![GollumPayload {
                                           page_name: "capitalisation".to_string(),
                                           title: "Capitalism".to_string(),
                                           action: "created".to_string(),
                                           sha: "4797f0ad2ee145181045fe69c61676e6".to_string(),
                                           html_url: "/capitalism".to_string(),
                                       },
                                       GollumPayload {
                                           page_name: "key".to_string(),
                                           title: "Keys".to_string(),
                                           action: "edited".to_string(),
                                           sha: "ce64d74910128530fad48dbd2bb4f836".to_string(),
                                           html_url: "/vodka".to_string(),
                                       }],
                       },
                       ..base()
                   }
                   .urls(),
               vec!["https://github.com/capitalism".to_string(), "https://github.com/vodka".to_string()]);
}

#[test]
fn issue_comment() {
    assert_eq!(Event {
                       payload: EventPayload::IssueComment {
                           action: "created".to_string(),
                           issue: 1,
                           body: "plz gib code".to_string(),
                           id: 4321,
                       },
                       ..base()
                   }
                   .urls(),
               vec!["https://github.com/nabijaczleweli/cargo-update/issues/1#issuecomment-4321".to_string()]);
}

#[test]
fn issues() {
    assert_eq!(Event {
                       payload: EventPayload::Issues {
                           action: "assigned".to_string(),
                           number: 2,
                           title: "This is code".to_string(),
                           body: "Closes #1".to_string(),
                           labels: vec!["invalid".to_string(), "question".to_string()],
                       },
                       ..base()
                   }
                   .urls(),
               vec!["https://github.com/nabijaczleweli/cargo-update/issues/2".to_string()]);
}

#[test]
fn member() {
    assert!(Event {
            payload: EventPayload::Member {
                action: "added".to_string(),
                user: "sehe".to_string(),
            },
            ..base()
        }
        .urls()
        .is_empty());
}

#[test]
fn public() {
    assert_eq!(Event { payload: EventPayload::Public, ..base() }.urls(),
               vec!["https://github.com/nabijaczleweli/cargo-update".to_string()]);
}

#[test]
fn pull_request() {
    assert_eq!(Event {
                       payload: EventPayload::PullRequest {
                           action: "assigned".to_string(),
                           number: 3,
                           title: "Remove bad numbar".to_string(),
                           body: "Closes #2".to_string(),
                           merged: false,
                       },
                       ..base()
                   }
                   .urls(),
               vec!["https://github.com/nabijaczleweli/cargo-update/pull/3".to_string()]);
}

#[test]
fn pull_request_review() {
    assert_eq!(Event {
                       payload: EventPayload::PullRequestReview {
                           action: "submitted".to_string(),
                           pr: 3,
                           state: "approved".to_string(),
                           body: ":+1:".to_string(),
                           id: 1243,
                       },
                       ..base()
                   }
                   .urls(),
               vec!["https://github.com/nabijaczleweli/cargo-update/pull/3#pullrequestreview-1243".to_string()]);
}

#[test]
fn pull_request_review_comment() {
    assert_eq!(Event {
                       payload: EventPayload::PullRequestReviewComment {
                           action: "created".to_string(),
                           pr: 3,
                           body: "This is bad :-1:".to_string(),
                           id: 4312,
                       },
                       ..base()
                   }
                   .urls(),
               vec!["https://github.com/nabijaczleweli/cargo-update/pull/3#discussion_r4312".to_string()]);
}

#[test]
fn push() {
    assert_eq!(Event {
                       payload: EventPayload::Push {
                           pushed_ref: "refs/heads/master".to_string(),
                           prev_head: "3c7ce203c3ca9317afa4beb51af4444c".to_string(),
                           new_head: "62476f13306db1cfade222d41bcdcb51".to_string(),
                           size: 10,
                           distinct_size: 9,
                           commits: vec![],
                       },
                       ..base()
                   }
                   .urls(),
               vec!["https://github.com/nabijaczleweli/cargo-update/compare/3c7ce203c3ca9317afa4beb51af4444c...62476f13306db1cfade222d41bcdcb51".to_string()]);
}

#[test]
fn release() {
    assert_eq!(Event {
                       payload: EventPayload::Release {
                           action: "published".to_string(),
                           tag_name: "v0.1.0".to_string(),
                           target: "master".to_string(),
                           draft: false,
                           prerelease: false,
                           name: None,
                           body: None,
                       },
                       ..base()
                   }
                   .urls(),
               vec!["https://github.com/nabijaczleweli/cargo-update/releases/tag/v0.1.0".to_string()]);
}

#[test]
fn watch() {
    assert_eq!(Event { payload: EventPayload::Watch { action: "started".to_string() }, ..base() }.urls(),
               vec!["https://github.com/nabijaczleweli/cargo-update/stargazers".to_string()]);
}

#[test]
fn other() {
    assert!(Event { payload: EventPayload::Other { event_type: "ForkApplyEvent".to_string() }, ..base() }.urls().is_empty());
}


fn base() -> Event {
    Event {
        created_at: DateTime::parse_from_rfc2822("Thu, 10 Nov 2016 00:42:18 +0000").unwrap(),
        actor: "liigo".to_string(),
        repo: "nabijaczleweli/cargo-update".to_string(),
        id: 4844096927,
        payload: EventPayload::Public,
    }
}
