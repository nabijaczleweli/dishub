//! This module contains functions which interface with the GitHub API.
//!
//! Every function in this module needs an instance of `AppTokens`, wherein the `discord` field does *not* need to be set.
//!
//! The GitHub authentication is used just to get a bigger rate limit,
//! so if you don't need to make a lot of requests just pass an empty string.


use hyper::header::{Authorization, IfNoneMatch, EntityTag, UserAgent, Bearer, ETag};
use self::super::super::util::GITHUB_USER_AGENT;
use hyper::status::StatusCode;
use self::super::super::Error;
use self::super::AppTokens;
use self::headers::*;
use hyper::Client;
use std::io::Read;


mod headers {
    header! {
        (XPollInterval, "X-Poll-Interval") => [u64]
    }
}


/// Check whether a user with the specified name exists.
///
/// # Examples
///
/// ```
/// # use dishub::ops::AppTokens;
/// # use dishub::ops::github::user_exists;
/// # let tokens = AppTokens {
/// #     github: "".to_string(),
/// #     discord: "".to_string(),
/// # };
/// let response = user_exists("nabijaczleweli", &tokens);
/// assert_eq!(response, Ok(true));
/// ```
pub fn user_exists(uname: &str, tokens: &AppTokens) -> Result<bool, Error> {
    exists(format!("https://api.github.com/users/{}", uname), tokens, "GitHub user information")
}

/// Check whether a repository with the specified slug exists.
///
/// # Examples
///
/// ```
/// # use dishub::ops::AppTokens;
/// # use dishub::ops::github::repo_exists;
/// # let tokens = AppTokens {
/// #     github: "".to_string(),
/// #     discord: "".to_string(),
/// # };
/// let response = repo_exists("nabijaczleweli/dishub", &tokens);
/// assert_eq!(response, Ok(true));
/// ```
pub fn repo_exists(slug: &str, tokens: &AppTokens) -> Result<bool, Error> {
    exists(format!("https://api.github.com/repos/{}", slug), tokens, "GitHub repository")
}

/// Get the events for a user when you don't have an ETag (which is to say - for the first time).
///
/// The returned tuple contains:
///
///   * The raw JSON response,
///   * The event bundle's ETag,
///   * The next minimum amount of milliseconds polling the same event queue is permitted.
///
/// You should use this only once and use `poll_user_events_update()` afterwards.
///
/// # Examples
///
/// ```no_run
/// # use dishub::ops::AppTokens;
/// # use dishub::ops::github::poll_user_events_new;
/// # let tokens = AppTokens {
/// #     github: "".to_string(),
/// #     discord: "".to_string(),
/// # };
/// let (response, etag, next) = poll_user_events_new("nabijaczleweli", &tokens).unwrap();
/// ```
pub fn poll_user_events_new(uname: &str, tokens: &AppTokens) -> Result<(String, String, u64), Error> {
    poll_events_new(format!("https://api.github.com/users/{}/events", uname), tokens, "GitHub user events")
}

/// Get the events for a repository when you don't have an ETag (which is to say - for the first time).
///
/// The returned tuple contains:
///
///   * The raw JSON response,
///   * The event bundle's ETag,
///   * The next minimum amount of milliseconds polling the same event queue is permitted.
///
/// You should use this only once and use `poll_repo_events_update()` afterwards.
///
/// # Examples
///
/// ```no_run
/// # use dishub::ops::AppTokens;
/// # use dishub::ops::github::poll_repo_events_new;
/// # let tokens = AppTokens {
/// #     github: "".to_string(),
/// #     discord: "".to_string(),
/// # };
/// let (response, etag, next) = poll_repo_events_new("nabijaczleweli/dishub", &tokens).unwrap();
/// ```
pub fn poll_repo_events_new(slug: &str, tokens: &AppTokens) -> Result<(String, String, u64), Error> {
    poll_events_new(format!("https://api.github.com/repos/{}/events", slug), tokens, "GitHub repo events")
}

/// Get the events for a user when you already have an ETag (which is to say - after the first time).
///
/// If the event list hasn't changed the first element of the returned tuple will be `None`,
/// otherwise it's a tuple of:
///
///   * The raw JSON response,
///   * The event bundle's new ETag.
///
/// The second element always constains the next minimum amount of milliseconds polling the same event queue is permitted.
///
/// # Examples
///
/// ```no_run
/// # use dishub::ops::AppTokens;
/// # use dishub::ops::github::poll_user_events_update;
/// # let tokens = AppTokens {
/// #     github: "".to_string(),
/// #     discord: "".to_string(),
/// # };
/// # let prev_etag = "9c1bac04e0735a8cba6a7b277b70c19f";
/// let (changed, next) = poll_user_events_update("nabijaczleweli", prev_etag, &tokens).unwrap();
/// if let Some((response, etag)) = changed {
///     // The feed changed
/// }
/// ```
pub fn poll_user_events_update(uname: &str, e_tag: &str, tokens: &AppTokens) -> Result<(Option<(String, String)>, u64), Error> {
    poll_events_update(format!("https://api.github.com/users/{}/events", uname), e_tag, tokens, "GitHub user events")
}

/// Get the events for a repository when you already have an ETag (which is to say - after the first time).
///
/// If the event list hasn't changed the first element of the returned tuple will be `None`,
/// otherwise it's a tuple of:
///
///   * The raw JSON response,
///   * The event bundle's new ETag.
///
/// The second element always constains the next minimum amount of milliseconds polling the same event queue is permitted.
///
/// # Examples
///
/// ```no_run
/// # use dishub::ops::AppTokens;
/// # use dishub::ops::github::poll_repo_events_update;
/// # let tokens = AppTokens {
/// #     github: "".to_string(),
/// #     discord: "".to_string(),
/// # };
/// # let prev_etag = "4797f0ad2ee145181045fe69c61676e6";
/// let (changed, next) = poll_repo_events_update("nabijaczleweli/dishub", prev_etag, &tokens).unwrap();
/// if let Some((response, etag)) = changed {
///     // The feed changed
/// }
/// ```
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
