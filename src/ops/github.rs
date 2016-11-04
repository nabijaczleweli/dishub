use hyper::header::{Authorization, UserAgent, Bearer};
use self::super::super::util::GITHUB_USER_AGENT;
use hyper::status::StatusCode;
use self::super::super::Error;
use self::super::AppTokens;
use hyper::Client;


pub fn user_exists(uname: &str, tokens: &AppTokens) -> Result<bool, Error> {
    exists(format!("https://api.github.com/users/{}", uname), tokens, "GitHub user information")
}

pub fn repo_exists(slug: &str, tokens: &AppTokens) -> Result<bool, Error> {
    exists(format!("https://api.github.com/repos/{}", slug), tokens, "GitHub repository")
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
