use chrono::{FixedOffset, DateTime};
use json::{self, JsonValue};


#[derive(Clone, Debug)]
pub struct Event {
    pub created_at: DateTime<FixedOffset>,
    pub actor: String,
    pub repo: String,
    pub payload: EventPayload,
}

#[derive(Clone, Debug)]
pub enum EventPayload {
    CommitComment {
        user: String,
        content: String,
        commit_id: String,
    },
    Create {
        ref_type: String,
        ref_name: Option<String>,
        master_branch: String,
        repo_description: String,
    },
    Delete { ref_type: String, ref_name: String, },
    Deployment {
        sha: String,
        payload: String,
        description: String,
    },
    Fork { new_slug: String, },
    Gollum { pages: Vec<GollumPayload>, },
    IssueComment {
        action: String,
        issue: u64,
        body: String,
    },
    Issue {
        action: String,
        number: u64,
        title: String,
        body: String,
        labels: Vec<String>,
    },
    Member { action: String, user: String, },
    Public,
    PullRequest {
        action: String,
        number: u64,
        title: String,
        body: String,
    },
    PullRequestReview {
        action: String,
        pr: u64,
        state: String,
        body: String,
    },
    PullRequestReviewComment {
        action: String,
        pr: u64,
        body: String,
    },
    Push {
        pushed_ref: String,
        prev_ref: String,
        size: u64,
        distinct_size: u64,
        commits: Vec<Commit>,
    },
    Release {
        action: String,
        tag_name: String,
        target: String,
        draft: bool,
        prerelease: bool,
        name: Option<String>,
        body: Option<String>,
    },
    Repository { action: String, slug: String, },
    Watch { action: String, slug: String, },
    Other(JsonValue),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct GollumPayload {
    page_name: String,
    title: String,
    action: String,
    sha: String,
    html_url: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Commit {
    sha: String,
    message: String,
    author_name: String,
    author_email: String,
    distinct: bool,
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
            "CommitCommentEvent" => {
                EventPayload::CommitComment {
                    user: ev["payload"]["user"]["login"].as_str().unwrap().to_string(),
                    content: ev["payload"]["body"].as_str().unwrap().to_string(),
                    commit_id: ev["payload"]["commit_id"].as_str().unwrap().to_string(),
                }
            }
            "CreateEvent" => {
                EventPayload::Create {
                    ref_type: ev["payload"]["ref_type"].as_str().unwrap().to_string(),
                    ref_name: ev["payload"]["ref"].as_str().map(str::to_string),
                    master_branch: ev["payload"]["master_branch"].as_str().unwrap().to_string(),
                    repo_description: ev["payload"]["description"].as_str().unwrap().to_string(),
                }
            }
            "DeleteEvent" => {
                EventPayload::Delete {
                    ref_type: ev["payload"]["ref_type"].as_str().unwrap().to_string(),
                    ref_name: ev["payload"]["ref"].as_str().unwrap().to_string(),
                }
            }
            "DeploymentEvent" => {
                EventPayload::Deployment {
                    sha: ev["payload"]["deployment"]["sha"].as_str().unwrap().to_string(),
                    payload: ev["payload"]["deployment"]["payload"].as_str().unwrap().to_string(),
                    description: ev["payload"]["deployment"]["deployment"].as_str().unwrap().to_string(),
                }
            }
            "ForkEvent" => EventPayload::Fork { new_slug: ev["payload"]["forkee"]["full_name"].as_str().unwrap().to_string() },
            "GollumEvent" => {
                EventPayload::Gollum {
                    pages: ev["payload"]["pages"]
                        .members()
                        .map(|pg| {
                            GollumPayload {
                                page_name: pg["page_name"].as_str().unwrap().to_string(),
                                title: pg["title"].as_str().unwrap().to_string(),
                                action: pg["action"].as_str().unwrap().to_string(),
                                sha: pg["sha"].as_str().unwrap().to_string(),
                                html_url: pg["html_url"].as_str().unwrap().to_string(),
                            }
                        })
                        .collect(),
                }
            }
            "IssueCommentEvent" => {
                EventPayload::IssueComment {
                    action: ev["payload"]["action"].as_str().unwrap().to_string(),
                    issue: ev["payload"]["issue"]["number"].as_number().unwrap().into(),
                    body: ev["payload"]["comment"]["body"].as_str().unwrap().to_string(),
                }
            }
            "IssueEvent" => {
                EventPayload::Issue {
                    action: ev["payload"]["action"].as_str().unwrap().to_string(),
                    number: ev["payload"]["issue"]["number"].as_number().unwrap().into(),
                    title: ev["payload"]["issue"]["title"].as_str().unwrap().to_string(),
                    body: ev["payload"]["issue"]["body"].as_str().unwrap().to_string(),
                    labels: ev["payload"]["issue"]["labels"].members().map(|l| l["name"].as_str().unwrap().to_string()).collect(),
                }
            }
            "MemberEvent" => {
                EventPayload::Member {
                    action: ev["payload"]["action"].as_str().unwrap().to_string(),
                    user: ev["payload"]["member"]["login"].as_str().unwrap().to_string(),
                }
            }
            "PublicEvent" => EventPayload::Public,
            "PullRequestEvent" => {
                EventPayload::PullRequest {
                    action: ev["payload"]["action"].as_str().unwrap().to_string(),
                    number: ev["payload"]["number"].as_number().unwrap().into(),
                    title: ev["payload"]["pull_request"]["title"].as_str().unwrap().to_string(),
                    body: ev["payload"]["pull_request"]["body"].as_str().unwrap().to_string(),
                }
            }
            "PullRequestReviewEvent" => {
                EventPayload::PullRequestReview {
                    action: ev["payload"]["action"].as_str().unwrap().to_string(),
                    pr: ev["payload"]["pull_request"]["number"].as_number().unwrap().into(),
                    state: ev["payload"]["review"]["state"].as_str().unwrap().to_string(),
                    body: ev["payload"]["review"]["body"].as_str().unwrap().to_string(),
                }
            }
            "PullRequestReviewCommentEvent" => {
                EventPayload::PullRequestReviewComment {
                    action: ev["payload"]["action"].as_str().unwrap().to_string(),
                    pr: ev["payload"]["pull_request"]["number"].as_number().unwrap().into(),
                    body: ev["payload"]["comment"]["body"].as_str().unwrap().to_string(),
                }
            }
            "PushEvent" => {
                EventPayload::Push {
                    pushed_ref: ev["payload"]["ref"].as_str().unwrap().to_string(),
                    prev_ref: ev["payload"]["before"].as_str().unwrap().to_string(),
                    size: ev["payload"]["size"].as_number().unwrap().into(),
                    distinct_size: ev["payload"]["distinct_size"].as_number().unwrap().into(),
                    commits: ev["payload"]["commits"]
                        .members()
                        .map(|c| {
                            Commit {
                                sha: c["sha"].as_str().unwrap().to_string(),
                                message: c["message"].as_str().unwrap().to_string(),
                                author_name: c["author"]["name"].as_str().unwrap().to_string(),
                                author_email: c["author"]["email"].as_str().unwrap().to_string(),
                                distinct: c["distinct"].as_bool().unwrap(),
                            }
                        })
                        .collect(),
                }
            }
            "ReleaseEvent" => {
                EventPayload::Release {
                    action: ev["payload"]["action"].as_str().unwrap().to_string(),
                    tag_name: ev["payload"]["tag_name"].as_str().unwrap().to_string(),
                    target: ev["payload"]["target_commitish"].as_str().unwrap().to_string(),
                    draft: ev["payload"]["draft"].as_bool().unwrap(),
                    prerelease: ev["payload"]["prerelease"].as_bool().unwrap(),
                    name: ev["payload"]["name"].as_str().map(str::to_string),
                    body: ev["payload"]["body"].as_str().map(str::to_string),
                }
            }
            "RepositoryEvent" => {
                EventPayload::Repository {
                    action: ev["payload"]["action"].as_str().unwrap().to_string(),
                    slug: ev["payload"]["repository"]["full_name"].as_str().unwrap().to_string(),
                }
            }
            "WatchEvent" => {
                EventPayload::Watch {
                    action: ev["payload"]["action"].as_str().unwrap().to_string(),
                    slug: ev["payload"]["repository"]["full_name"].as_str().unwrap().to_string(),
                }
            }
            _ => EventPayload::Other(ev.clone()),
        }
    }
}
