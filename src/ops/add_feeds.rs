use self::super::super::util::prompt_nonzero_len;
use self::super::{AppTokens, github, verify_file};
use self::super::super::Error;
use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::fs::File;


pub fn verify(config_dir: &(String, PathBuf)) -> Result<(PathBuf, PathBuf), Error> {
    let tokens = try!(verify_file("tokens.toml", true, config_dir, false, "init"));
    let feeds = try!(verify_file("feeds.toml", true, config_dir, true, ""));

    File::create(&feeds).unwrap();

    Ok((tokens, feeds))
}

pub fn get_watch_subject<R: BufRead, W: Write>(input: &mut R, output: &mut W) -> String {
    prompt_nonzero_len(input,
                       output,
                       "What to watch (repo slug or user)",
                       |s| s.chars().filter(|&c| c == '/').count() <= 1)
        .unwrap()
}

pub fn verify_subject(subject: &str, tokens: &AppTokens) -> Result<(), Error> {
    if subject.contains('/') {
        do_verify(subject, tokens, github::repo_exists, "repository")
    } else {
        do_verify(subject, tokens, github::user_exists, "user")
    }
}

fn do_verify(subject: &str, tokens: &AppTokens, check_fn: fn(&str, &AppTokens) -> Result<bool, Error>, tp: &'static str) -> Result<(), Error> {
    check_fn(subject, tokens).and_then(|e| if e {
        Ok(())
    } else {
        Err(Error::WatchedDoesNotExist {
            tp: tp,
            name: subject.to_string(),
        })
    })
}
