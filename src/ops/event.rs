use chrono::{FixedOffset, DateTime};
use json::{self, JsonValue};
use std::fmt;


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
        content: String,
        commit_id: String,
        id: u64,
    },
    Create {
        ref_type: String,
        ref_name: Option<String>,
        master_branch: String,
        repo_description: String,
    },
    Delete { ref_type: String, ref_name: String, },
    Fork { new_slug: String, },
    Gollum { pages: Vec<GollumPayload>, },
    IssueComment {
        action: String,
        issue: u64,
        body: String,
        id: u64,
    },
    Issues {
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
        id: u64,
    },
    PullRequestReviewComment {
        action: String,
        pr: u64,
        body: String,
        id: u64,
    },
    Push {
        pushed_ref: String,
        prev_head: String,
        new_head: String,
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
    Watch { action: String, },
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

    pub fn urls(&self) -> Vec<String> {
        match self.payload {
            EventPayload::CommitComment { ref commit_id, id, .. } => {
                vec![format!("https://github.com/{}/commit/{}#commitcomment-{}", self.repo, commit_id, id)]
            }
            EventPayload::Create { ref ref_name, .. } => {
                match *ref_name {
                    Some(ref ref_name) => vec![format!("https://github.com/{}/compare/{}", self.repo, ref_name)],
                    None => vec![format!("https://github.com/{}", self.repo)],
                }
            }
            EventPayload::Delete { .. } => vec![],
            EventPayload::Fork { ref new_slug } => vec![format!("https://github.com/{}", new_slug)],
            EventPayload::Gollum { ref pages } => pages.iter().map(|p| p.html_url.clone()).collect(),
            EventPayload::IssueComment { issue, id, .. } => vec![format!("https://github.com/{}/issues/{}#issuecomment-{}", self.repo, issue, id)],
            EventPayload::Issues { number, .. } => vec![format!("https://github.com/{}/issues/{}", self.repo, number)],
            EventPayload::Member { .. } => vec![],
            EventPayload::Public => vec![format!("https://github.com/{}", self.repo)],
            EventPayload::PullRequest { number, .. } => vec![format!("https://github.com/{}/pull/{}", self.repo, number)],
            EventPayload::PullRequestReview { pr, id, .. } => {
                // TODO: is this the correct URL for the review au general?
                vec![format!("https://github.com/{}/pull/{}#discussion_r{}", self.repo, pr, id)]
            }
            EventPayload::PullRequestReviewComment { pr, id, .. } => vec![format!("https://github.com/{}/pull/{}#discussion_r{}", self.repo, pr, id)],
            EventPayload::Push { ref prev_head, ref new_head, .. } => vec![format!("https://github.com/{}/compare/{}...{}", self.repo, prev_head, new_head)],
            EventPayload::Release { ref tag_name, .. } => vec![format!("https://github.com/{}/releases/tag/{}", self.repo, tag_name)],
            EventPayload::Watch { .. } => vec![format!("https://github.com/{}/stargazers", self.repo)],
            EventPayload::Other(_) => vec![],
        }
    }
}

impl EventPayload {
    pub fn from(ev: &JsonValue) -> EventPayload {
        match ev["type"].as_str().unwrap() {
            "CommitCommentEvent" => {
                EventPayload::CommitComment {
                    content: ev["payload"]["comment"]["body"].as_str().unwrap().to_string(),
                    commit_id: ev["payload"]["comment"]["commit_id"].as_str().unwrap().to_string(),
                    id: ev["payload"]["comment"]["id"].as_number().unwrap().into(),
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
                    id: ev["payload"]["comment"]["id"].as_number().unwrap().into(),
                }
            }
            "IssuesEvent" => {
                EventPayload::Issues {
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
                    id: ev["payload"]["review"]["id"].as_number().unwrap().into(),
                }
            }
            "PullRequestReviewCommentEvent" => {
                EventPayload::PullRequestReviewComment {
                    action: ev["payload"]["action"].as_str().unwrap().to_string(),
                    pr: ev["payload"]["pull_request"]["number"].as_number().unwrap().into(),
                    body: ev["payload"]["comment"]["body"].as_str().unwrap().to_string(),
                    id: ev["payload"]["comment"]["id"].as_number().unwrap().into(),
                }
            }
            "PushEvent" => {
                EventPayload::Push {
                    pushed_ref: ev["payload"]["ref"].as_str().unwrap().to_string(),
                    prev_head: ev["payload"]["before"].as_str().unwrap().to_string(),
                    new_head: ev["payload"]["head"].as_str().unwrap().to_string(),
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
                    tag_name: ev["payload"]["release"]["tag_name"].as_str().unwrap().to_string(),
                    target: ev["payload"]["release"]["target_commitish"].as_str().unwrap().to_string(),
                    draft: ev["payload"]["release"]["draft"].as_bool().unwrap(),
                    prerelease: ev["payload"]["release"]["prerelease"].as_bool().unwrap(),
                    name: ev["payload"]["release"]["name"].as_str().map(str::to_string),
                    body: ev["payload"]["release"]["body"].as_str().map(str::to_string),
                }
            }
            "WatchEvent" => EventPayload::Watch { action: ev["payload"]["action"].as_str().unwrap().to_string() },
            _ => EventPayload::Other(ev.clone()),
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}: ", self.created_at.format("%d.%m.%y %r")));

        match self.payload {
            EventPayload::CommitComment { ref commit_id, .. } => {
                try!(write!(f, "{} commented on {} in {}", self.actor, commit_id, self.repo));
            }
            EventPayload::Create { ref ref_type, ref ref_name, ref repo_description, .. } => {
                try!(write!(f, "{} created {}", self.actor, ref_type));
                if let Some(ref ref_name) = *ref_name {
                    try!(write!(f, " {}", ref_name));
                } else {
                    try!(write!(f, " \"{}\"", repo_description));
                }
            }
            EventPayload::Delete { ref ref_type, ref ref_name } => {
                try!(write!(f, "{} deleted {} {}", self.actor, ref_type, ref_name));
            }
            EventPayload::Fork { ref new_slug } => {
                try!(write!(f, "{} forked {} to {}", self.actor, self.repo, new_slug));
            }
            EventPayload::Gollum { ref pages } => {
                try!(write!(f, "{} changed wiki on {}:", self.actor, self.repo));
                for (i, &GollumPayload { ref title, ref action, .. }) in pages.iter().enumerate() {
                    if i != pages.len() - 1 {
                        try!(writeln!(f, ""));
                    }
                    try!(write!(f, "  {} \"{}\"", action, title));
                }
            }
            EventPayload::IssueComment { ref action, issue, .. } => {
                try!(write!(f, "{} {} comment to #{} on {}", self.actor, action, issue, self.repo));
            }
            EventPayload::Issues { ref action, number, ref title, .. } => {
                try!(write!(f, "{} {} #{} on {}: \"{}\"", self.actor, action, number, self.repo, title));
            }
            EventPayload::Member { ref action, ref user } => {
                try!(write!(f, "{} {} {} to {}", self.actor, action, user, self.repo));
            }
            EventPayload::Public => {
                try!(write!(f, "{} made {} public", self.actor, self.repo));
            }
            EventPayload::PullRequest { ref action, number, ref title, .. } => {
                try!(write!(f, "{} {} #{} on {}: \"{}\"", self.actor, action, number, self.repo, title));
            }
            EventPayload::PullRequestReview { ref action, pr, ref state, .. } => {
                try!(write!(f, "{} {} as {} #{} on {}", self.actor, action, state, pr, self.repo));
            }
            EventPayload::PullRequestReviewComment { ref action, pr, .. } => {
                try!(write!(f, "{} {} #{} on {}", self.actor, action, pr, self.repo));
            }
            EventPayload::Push { ref pushed_ref, distinct_size, .. } => {
                try!(write!(f,
                            "{} pushed {} commit{} to {} in {}",
                            self.actor,
                            distinct_size,
                            if distinct_size != 1 { "s" } else { "" },
                            pushed_ref.split('/').last().unwrap(),
                            self.repo));
            }
            EventPayload::Release { ref action, ref tag_name, ref target, draft, prerelease, ref name, .. } => {
                try!(write!(f, "{} {} {} from {}", self.actor, action, tag_name, target));
                if draft {
                    try!(write!(f, " as a draft"));
                } else if prerelease {
                    try!(write!(f, " as a prerelease"));
                }
                if let Some(ref name) = *name {
                    try!(write!(f, " named {}", name));
                }
            }
            EventPayload::Watch { .. } => {
                try!(write!(f, "{} starred {}", self.actor, self.repo));
            }
            EventPayload::Other(ref ev) => {
                try!(write!(f, "unsupported event: {}", ev["type"]));
            }
        }

        Ok(())
    }
}
