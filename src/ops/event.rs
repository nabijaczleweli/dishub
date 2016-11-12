use chrono::{FixedOffset, DateTime};
use json::{self, JsonValue};
use std::str::FromStr;
use std::fmt;


/// A representation of the [GitHub Event API's](https://developer.github.com/v3/activity/events) event.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Event {
    /// The time this event was generated.
    ///
    /// Corresponds to `created_at` in the event JSON.
    pub created_at: DateTime<FixedOffset>,
    /// The person who triggered the event.
    ///
    /// Corresponds to `actor.display_login` in the event JSON.
    pub actor: String,
    /// The repository slug where the event is triggered.
    ///
    /// Corresponds to `repo.name` in the event JSON.
    pub repo: String,
    /// The event ID.
    ///
    /// Corresponds to `id` in the event JSON (except it's a string there).
    pub id: u64,
    /// The event's payload.
    ///
    /// Corresponds to `payload` in the event JSON, determined by `type`.
    pub payload: EventPayload,
}

/// A representation of the GitHub Event API's [event payload](https://developer.github.com/v3/activity/events/types).
///
/// We only represent the event types that are visible in timelines and haven't been phased out yet.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EventPayload {
    /// A [CommitCommentEvent](https://developer.github.com/v3/activity/events/types#commitcommentevent).
    CommitComment {
        /// The comment's body.
        content: String,
        /// The commit SHA.
        commit_id: String,
        /// The comment's ID.
        id: u64,
    },
    /// A [CreateEvent](https://developer.github.com/v3/activity/events/types#createevent).
    Create {
        /// The ref's type.
        ///
        /// Can be "repository", "branch", or "tag".
        ref_type: String,
        /// The ref's name.
        ///
        /// `None` if only a repository was created.
        ref_name: Option<String>,
        /// The master branch of the repository.
        master_branch: String,
        /// The repository's description (the text right under the tab list).
        repo_description: String,
    },
    /// A [DeleteEvent](https://developer.github.com/v3/activity/events/types#deleteevent).
    Delete {
        /// The deleted ref's type.
        ///
        /// Can be "branch" or "tag".
        ref_type: String,
        /// The deleted ref's name.
        ref_name: String,
    },
    /// A [ForkEvent](https://developer.github.com/v3/activity/events/types#forkevent).
    Fork {
        /// The fork's slug.
        new_slug: String,
    },
    /// A [GollumEvent](https://developer.github.com/v3/activity/events/types#gollumevent) (a.k.a. a WikiEvent).
    Gollum {
        /// The affected Wiki pages.
        pages: Vec<GollumPayload>,
    },
    /// An [IssueCommentEvent](https://developer.github.com/v3/activity/events/types#issuecommentevent).
    IssueComment {
        /// The action executed upon a comment.
        ///
        /// Can be "created", "edited", or "deleted".
        action: String,
        /// The issue number.
        issue: u64,
        /// The issue comment's body text.
        body: String,
        /// The issue comment's ID.
        id: u64,
    },
    /// An [IssuesEvent](https://developer.github.com/v3/activity/events/types#issuesevent).
    Issues {
        /// The action executed upon an issue.
        ///
        /// Can be "assigned", "unassigned", "labeled", "unlabeled", "opened", "edited", "milestoned", "demilestoned",
        /// "closed", or "reopened".
        action: String,
        /// The issue number.
        number: u64,
        /// The issue's title.
        title: String,
        /// The issue body's text.
        body: String,
        /// The labels the issue has upon it.
        labels: Vec<String>,
    },
    /// A [MemberEvent](https://developer.github.com/v3/activity/events/types#memberevent).
    Member {
        /// The action executed upon a user.
        ///
        /// Can be only "added".
        action: String,
        /// The user the action was performed upon.
        user: String,
    },
    /// A [PublicEvent](https://developer.github.com/v3/activity/events/types#publicevent).
    ///
    /// A repository was made public.
    Public,
    /// A [PullRequestEvent](https://developer.github.com/v3/activity/events/types#pullrequestevent).
    PullRequest {
        /// The action executed upon a PR.
        ///
        /// Can be "assigned", "unassigned", "labeled", "unlabeled", "opened", "edited", "closed", or "reopened".
        ///
        /// If the action is "closed" and the `merged` is `false`, the pull request was closed with
        /// unmerged commits. If the action is "closed" and `merged` is `true`, the pull request was merged.
        action: String,
        /// The PR number.
        number: u64,
        /// The PR's title.
        title: String,
        /// The PR body's content.
        body: String,
        /// Whether the PR was merged.
        ///
        /// If the action is "closed" and the `merged` is `false`, the pull request was closed with
        /// unmerged commits. If the action is "closed" and `merged` is `true`, the pull request was merged.
        merged: bool,
    },
    /// A [PullRequestReviewEvent](https://developer.github.com/v3/activity/events/types#pullrequestreviewevent).
    PullRequestReview {
        /// The action executed upon a PR review.
        ///
        /// Can be only "submitted".
        action: String,
        /// The PR number.
        pr: u64,
        /// The PR review's state.
        ///
        /// Something like "approved".
        state: String,
        /// The PR review's body.
        body: String,
        /// The PR review's ID.
        id: u64,
    },
    /// A [PullRequestReviewCommentEvent](https://developer.github.com/v3/activity/events/types#pullrequestreviewcommentevent).
    PullRequestReviewComment {
        /// The action executed upon a PR review comment.
        ///
        /// Can be "created", "edited", or "deleted".
        action: String,
        /// The PR number.
        pr: u64,
        /// The PR review comment's body.
        body: String,
        /// The PR review comment's ID.
        id: u64,
    },
    /// A [PushEvent](https://developer.github.com/v3/activity/events/types#pushevent).
    Push {
        /// The full Git ref pushed to.
        pushed_ref: String,
        /// The previous HEAD's SHA.
        prev_head: String,
        /// The new HEAD's SHA.
        new_head: String,
        /// The amount of commits pushed.
        size: u64,
        /// The amount of distinct commits pushed.
        distinct_size: u64,
        /// The commits pushed.
        commits: Vec<Commit>,
    },
    /// A [ReleaseEvent](https://developer.github.com/v3/activity/events/types#releaseevent).
    Release {
        /// The action executed upon a release.
        ///
        /// Can be only "published".
        action: String,
        /// The released tag's name.
        tag_name: String,
        /// The released branch's name.
        target: String,
        /// Whether the release is a draft.
        draft: bool,
        /// Whether the release is a prerelease.
        prerelease: bool,
        /// The release's custom name.
        ///
        /// This is `None` if the release is just a tag and not edited through the GitHub release editor.
        name: Option<String>,
        /// The release's custom body.
        ///
        /// This is `None` if the release is just a tag and not edited through the GitHub release editor.
        body: Option<String>,
    },
    /// A [WatchEvent](https://developer.github.com/v3/activity/events/types#watchevent), or, more aptly, a StarEvent.
    Watch {
        /// The action executed upon a repository star.
        ///
        /// Can only be "started".
        action: String,
    },
    /// An unhandled event.
    Other {
        /// The event type.
        event_type: String,
    },
}

/// A Wiki page affected by a [GollumEvent](https://developer.github.com/v3/activity/events/types#gollumevent).
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct GollumPayload {
    /// The Wiki page's name.
    pub page_name: String,
    /// The Wiki page's current title.
    pub title: String,
    /// The action executed upon a Wiki page.
    ///
    /// Can be "created" or "edited".
    pub action: String,
    /// The latest commit SHA of the Wiki page.
    pub sha: String,
    /// The URL to the Wiki page.
    ///
    /// Can be used straight as a user-facing URL.
    pub html_url: String,
}

/// A Git commit pushed in a [PushEvent](https://developer.github.com/v3/activity/events/types#pushevent).
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Commit {
    /// The commit's SHA.
    pub sha: String,
    /// The commit message.
    pub message: String,
    /// The commit author's name.
    ///
    /// GitHub Event API doesn't give us the committer data.
    pub author_name: String,
    /// The commit author's e-mail.
    ///
    /// GitHub Event API doesn't give us the committer data.
    pub author_email: String,
    /// Whether the commit is distinct.
    pub distinct: bool,
}


impl Event {
    /// Parse a raw JSON GitHub Events API response.
    ///
    /// The JSON is not checked for correctness, so be wary.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate chrono;
    /// # extern crate dishub;
    /// # use dishub::ops::{EventPayload, Event};
    /// # use chrono::DateTime;
    /// # fn main() {
    /// // Shaved to minimum for brevity
    /// let response = r#"[
    ///                     {
    ///                       "id": "4831774905",
    ///                       "type": "WatchEvent",
    ///                       "actor": {
    ///                         "display_login": "carllhw"
    ///                       },
    ///                       "repo": {
    ///                         "name": "nabijaczleweli/cargo-update"
    ///                       },
    ///                       "payload": {
    ///                         "action": "started"
    ///                       },
    ///                       "created_at": "2016-11-08T03:10:26Z"
    ///                     },
    ///                     {
    ///                       "id": "4831775201",
    ///                       "type": "WatchEvent",
    ///                       "actor": {
    ///                         "display_login": "Byron-TW"
    ///                       },
    ///                       "repo": {
    ///                         "name": "nabijaczleweli/cargo-update"
    ///                       },
    ///                       "payload": {
    ///                         "action": "started"
    ///                       },
    ///                       "created_at": "2016-11-09T06:12:26Z"
    ///                     }
    ///                   ]"#;
    /// assert_eq!(Event::parse(response), vec![
    ///            Event {
    ///                created_at: DateTime::parse_from_rfc2822("Tue, 8 Nov 2016 03:10:26 +0000").unwrap(),
    ///                actor: "carllhw".to_string(),
    ///                repo: "nabijaczleweli/cargo-update".to_string(),
    ///                id: 4831774905,
    ///                payload: EventPayload::Watch {
    ///                    action: "started".to_string(),
    ///                },
    ///            },
    ///            Event {
    ///                created_at: DateTime::parse_from_rfc2822("Wed, 9 Nov 2016 06:12:26 +0000").unwrap(),
    ///                actor: "Byron-TW".to_string(),
    ///                repo: "nabijaczleweli/cargo-update".to_string(),
    ///                id: 4831775201,
    ///                payload: EventPayload::Watch {
    ///                    action: "started".to_string(),
    ///                },
    ///            }
    ///        ]);
    /// # }
    /// ```
    pub fn parse(what: &str) -> Vec<Event> {
        json::parse(what)
            .unwrap()
            .members()
            .map(|j| {
                Event {
                    created_at: DateTime::parse_from_rfc3339(j["created_at"].as_str().unwrap()).unwrap(),
                    actor: j["actor"]["display_login"].as_str().unwrap().to_string(),
                    repo: j["repo"]["name"].as_str().unwrap().to_string(),
                    id: u64::from_str(j["id"].as_str().unwrap()).unwrap(),
                    payload: EventPayload::from(j),
                }
            })
            .collect()
    }

    /// Get the reference URLs for an event.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate chrono;
    /// # extern crate dishub;
    /// # use dishub::ops::{EventPayload, Event};
    /// # use chrono::DateTime;
    /// # fn main() {
    /// // Shaved to minimum for brevity
    /// let response = r#"[{
    ///                     "id": "4831774905",
    ///                     "type": "WatchEvent",
    ///                     "actor": { "display_login": "carllhw" },
    ///                     "repo": { "name": "nabijaczleweli/cargo-update" },
    ///                     "payload": { "action": "started" },
    ///                     "created_at": "2016-11-08T03:10:26Z"
    ///                   }]"#;
    /// assert_eq!(Event::parse(response)[0].urls(),
    ///            vec!["https://github.com/nabijaczleweli/cargo-update/stargazers".to_string()]);
    /// # }
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
            EventPayload::PullRequestReview { pr, id, .. } => vec![format!("https://github.com/{}/pull/{}#pullrequestreview-{}", self.repo, pr, id)],
            EventPayload::PullRequestReviewComment { pr, id, .. } => vec![format!("https://github.com/{}/pull/{}#discussion_r{}", self.repo, pr, id)],
            EventPayload::Push { ref prev_head, ref new_head, .. } => vec![format!("https://github.com/{}/compare/{}...{}", self.repo, prev_head, new_head)],
            EventPayload::Release { ref tag_name, .. } => vec![format!("https://github.com/{}/releases/tag/{}", self.repo, tag_name)],
            EventPayload::Watch { .. } => vec![format!("https://github.com/{}/stargazers", self.repo)],
            EventPayload::Other { .. } => vec![],
        }
    }
}

impl EventPayload {
    /// Parse the payload from the *whole* event.
    ///
    /// Not meant to be used directly, use `Event::parse()` instead.
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
                    merged: ev["payload"]["pull_request"]["merged"].as_bool().unwrap(),
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
            t => EventPayload::Other { event_type: t.to_string() },
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}: ", self.created_at.format("%d.%m.%Y %r")));

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
            EventPayload::PullRequest { ref action, number, ref title, merged, .. } => {
                try!(write!(f,
                            "{} {} #{} on {}: \"{}\"",
                            self.actor,
                            if merged && action == "closed" {
                                "merged"
                            } else {
                                action
                            },
                            number,
                            self.repo,
                            title));
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
            EventPayload::Other { ref event_type } => {
                try!(write!(f, "unsupported event: {}", event_type));
            }
        }

        Ok(())
    }
}
