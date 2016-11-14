use json;
use dishub::ops::{GollumPayload, EventPayload, Commit};


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
    assert_eq!(EventPayload::from("CommitCommentEvent", &json::parse(COMMIT_COMMENT).unwrap()),
               EventPayload::CommitComment {
                   content: "> maybe on your shit thing\r\n".to_string(),
                   commit_id: "1ce17f668485fd6741895c3caba1f2ea77ab0e6c".to_string(),
                   id: 19779422,
               });
}

#[test]
fn create() {
    assert_eq!(EventPayload::from("CreateEvent", &json::parse(CREATE).unwrap()),
               EventPayload::Create {
                   ref_type: "tag".to_string(),
                   ref_name: Some("v0.5.0".to_string()),
                   master_branch: "master".to_string(),
                   repo_description: "A cargo subcommand for checking and applying updates to installed executables".to_string(),
               });
}

#[test]
fn delete() {
    assert_eq!(EventPayload::from("DeleteEvent", &json::parse(DELETE).unwrap()),
               EventPayload::Delete {
                   ref_type: "tag".to_string(),
                   ref_name: "v0.2.0".to_string(),
               });
}

#[test]
fn fork() {
    assert_eq!(EventPayload::from("ForkEvent", &json::parse(FORK).unwrap()),
               EventPayload::Fork { new_slug: "nabijaczleweli/clap-rs".to_string() });
}

#[test]
fn gollum() {
    assert_eq!(EventPayload::from("GollumEvent", &json::parse(GOLLUM).unwrap()),
               EventPayload::Gollum {
                   pages: vec![GollumPayload {
                                   page_name: "Packages".to_string(),
                                   title: "Packages".to_string(),
                                   action: "edited".to_string(),
                                   sha: "bf842e3aa1fad56ef96e0256eccb3d9d8f288aab".to_string(),
                                   html_url: "/clibs/clib/wiki/Packages".to_string(),
                               },
                               GollumPayload {
                                   page_name: "Packages".to_string(),
                                   title: "Packages".to_string(),
                                   action: "edited".to_string(),
                                   sha: "c2685b6d3712605276a37a99022e6df89a29595e".to_string(),
                                   html_url: "/clibs/clib/wiki/Packages".to_string(),
                               }],
               });
}

#[test]
fn issue_comment() {
    assert_eq!(EventPayload::from("IssueCommentEvent", &json::parse(ISSUE_COMMENT).unwrap()),
               EventPayload::IssueComment {
                   action: "created".to_string(),
                   issue: 12,
                   body: "It is possible to install the same package twice".to_string(),
                   id: 259662246,
               });
}

#[test]
fn issues() {
    assert_eq!(EventPayload::from("IssuesEvent", &json::parse(ISSUES).unwrap()),
               EventPayload::Issues {
                   action: "opened".to_string(),
                   number: 11,
                   title: "'unknown error occurred': It".to_string(),
                   body: r"```C:\Users\liigo>cargo install-update```".to_string(),
                   labels: vec!["bug".to_string(), "question".to_string()],
               });
}

#[test]
fn member() {
    assert_eq!(EventPayload::from("MemberEvent", &json::parse(MEMBER).unwrap()),
               EventPayload::Member {
                   action: "added".to_string(),
                   user: "Enet4".to_string(),
               });
}

#[test]
fn public() {
    assert_eq!(EventPayload::from("PublicEvent", &json::parse(PUBLIC).unwrap()), EventPayload::Public);
}

#[test]
fn pull_request() {
    assert_eq!(EventPayload::from("PullRequestEvent", &json::parse(PULL_REQUEST).unwrap()),
               EventPayload::PullRequest {
                   action: "closed".to_string(),
                   number: 138,
                   title: "bumping version.h to 1.7.0".to_string(),
                   body: "".to_string(),
                   merged: true,
               });
}

#[test]
fn pull_request_review() {
    assert_eq!(EventPayload::from("PullRequestReviewEvent", &json::parse(PULL_REQUEST_REVIEW).unwrap()),
               EventPayload::PullRequestReview {
                   action: "submitted".to_string(),
                   pr: 210,
                   state: "approved".to_string(),
                   body: "Very Good!".to_string(),
                   id: 48126498,
               });
}

#[test]
fn pull_request_review_comment() {
    assert_eq!(EventPayload::from("PullRequestReviewCommentEvent", &json::parse(PULL_REQUEST_REVIEW_COMMENT).unwrap()),
               EventPayload::PullRequestReviewComment {
                   action: "created".to_string(),
                   pr: 210,
                   body: "Not Very Good, but :+1:".to_string(),
                   id: 63127893,
               });
}

#[test]
fn push() {
    assert_eq!(EventPayload::from("PushEvent", &json::parse(PUSH).unwrap()),
               EventPayload::Push {
                   pushed_ref: "refs/heads/doc".to_string(),
                   prev_head: "4d28f4b488f04c35135af7576ff5fd1f0ce53c7a".to_string(),
                   new_head: "46650797f4aec4d373c647062eb3df288ee7b8f2".to_string(),
                   size: 2,
                   distinct_size: 1,
                   commits: vec![Commit {
                                     sha: "46650797f4aec4d373c647062eb3df288ee7b8f2".to_string(),
                                     message: "Update docs for commits f69e263c6caf...40e07efdf879".to_string(),
                                     author_name: "Nabijaczleweli Autouploader Bot".to_string(),
                                     author_email: "nabijaczleweli@gmail.com".to_string(),
                                     distinct: true,
                                 },
                                 Commit {
                                     sha: "8eab6892203102e66a4b1eed26abc5a24cad8afe".to_string(),
                                     message: "Update manual for commits 1c8725dc2222...58998fadba13".to_string(),
                                     author_name: "Nabijaczleweli Autouploader Bot".to_string(),
                                     author_email: "nabijaczleweli@gmail.com".to_string(),
                                     distinct: false,
                                 }],
               });
}

#[test]
fn release() {
    assert_eq!(EventPayload::from("ReleaseEvent", &json::parse(RELEASE).unwrap()),
               EventPayload::Release {
                   action: "published".to_string(),
                   tag_name: "v0.5.0".to_string(),
                   target: "master".to_string(),
                   draft: false,
                   prerelease: false,
                   name: None,
                   body: None,
               });
}

#[test]
fn watch() {
    assert_eq!(EventPayload::from("WatchEvent", &json::parse(WATCH).unwrap()),
               EventPayload::Watch { action: "started".to_string() });
}

#[test]
fn other() {
    assert_eq!(EventPayload::from("ForkApplyEvent", &json::parse(OTHER_FORK_APPLY).unwrap()),
               EventPayload::Other { event_type: "ForkApplyEvent".to_string() });
}
