use hyper::header::{Authorization, IfNoneMatch, EntityTag, UserAgent, Bearer, ETag};
use self::super::super::util::GITHUB_USER_AGENT;
use hyper::status::StatusCode;
use self::super::super::Error;
use self::super::AppTokens;
use hyper::Client;
use std::io::Read;


header! {
    (XPollInterval, "X-Poll-Interval") => [u64]
}


pub fn user_exists(uname: &str, tokens: &AppTokens) -> Result<bool, Error> {
    exists(format!("https://api.github.com/users/{}", uname), tokens, "GitHub user information")
}

pub fn repo_exists(slug: &str, tokens: &AppTokens) -> Result<bool, Error> {
    exists(format!("https://api.github.com/repos/{}", slug), tokens, "GitHub repository")
}

pub fn poll_user_events_new(uname: &str, tokens: &AppTokens) -> Result<(String, String, u64), Error> {
    poll_events_new(format!("https://api.github.com/users/{}/events", uname), tokens, "GitHub user events")
}

pub fn poll_repo_events_new(slug: &str, tokens: &AppTokens) -> Result<(String, String, u64), Error> {
    poll_events_new(format!("https://api.github.com/repos/{}/events", slug), tokens, "GitHub repo events")
}

pub fn poll_user_events_update(uname: &str, e_tag: &str, tokens: &AppTokens) -> Result<(Option<(String, String)>, u64), Error> {
    poll_events_update(format!("https://api.github.com/users/{}/events", uname), e_tag, tokens, "GitHub user events")
}

pub fn poll_repo_events_update(slug: &str, e_tag: &str, tokens: &AppTokens) -> Result<(Option<(String, String)>, u64), Error> {
    poll_events_update(format!("https://api.github.com/repos/{}/events", slug), e_tag, tokens, "GitHub user events")
}

fn exists(url: String, tokens: &AppTokens, desc: &'static str) -> Result<bool, Error> {
    Client::new()
        .get(&url)
        .header(Authorization(Bearer { token: tokens.github.clone() }))
        .header(UserAgent(GITHUB_USER_AGENT.to_string()))
        .send()
        .map_err(|_| {
            Error::Io {
                desc: desc,
                op: "get",
            }
        })
        .map(|r| r.status != StatusCode::NotFound)
}

fn poll_events_new(url: String, tokens: &AppTokens, desc: &'static str) -> Result<(String, String, u64), Error> {
    Client::new()
        .get(&url)
        .header(Authorization(Bearer { token: tokens.github.clone() }))
        .header(UserAgent(GITHUB_USER_AGENT.to_string()))
        .send()
        .map_err(|_| {
            Error::Io {
                desc: desc,
                op: "poll",
            }
        })
        .map(|mut r| {
            let mut buf = String::new();
            r.read_to_string(&mut buf).unwrap();

            let etag: &ETag = r.headers.get().unwrap();
            let poll_interval: &XPollInterval = r.headers.get().unwrap();
            (buf, etag.tag().to_string(), **poll_interval)
        })
}

fn poll_events_update(url: String, etag: &str, tokens: &AppTokens, desc: &'static str) -> Result<(Option<(String, String)>, u64), Error> {
    Client::new()
        .get(&url)
        .header(Authorization(Bearer { token: tokens.github.clone() }))
        .header(UserAgent(GITHUB_USER_AGENT.to_string()))
        .header(IfNoneMatch::Items(vec![EntityTag::new(false, etag.to_string())]))
        .send()
        .map_err(|_| {
            Error::Io {
                desc: desc,
                op: "poll",
            }
        })
        .map(|mut r| {
            (if r.status == StatusCode::NotModified {
                None
            } else {
                let mut buf = String::new();
                r.read_to_string(&mut buf).unwrap();

                let etag: &ETag = r.headers.get().unwrap();
                Some((buf, etag.tag().to_string()))
            },
             r.headers.get::<XPollInterval>().map(|r| **r).unwrap_or(60))
        })
}
