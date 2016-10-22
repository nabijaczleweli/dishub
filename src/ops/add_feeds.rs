use self::super::{AppTokens, Feed, github, verify_file};
use self::super::super::util::prompt_nonzero_len;
use self::super::super::Error;
use std::io::{BufRead, Write};
use discord::{State, Discord};
use discord::model::ServerId;
use std::path::PathBuf;
use std::str::FromStr;


pub fn verify(config_dir: &(String, PathBuf)) -> Result<(PathBuf, PathBuf), Error> {
    let tokens = try!(verify_file("tokens.toml", true, config_dir, false, "init"));
    let feeds = try!(verify_file("feeds.toml", true, config_dir, true, ""));

    if !feeds.exists() {
        Feed::write(vec![], &feeds);
    }

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
    let (check_fn, tp): (fn(&str, &AppTokens) -> Result<bool, Error>, &'static str) = if subject.contains('/') {
        (github::repo_exists, "repository")
    } else {
        (github::user_exists, "user")
    };

    check_fn(subject, tokens).and_then(|e| if e {
        Ok(())
    } else {
        Err(Error::WatchedDoesNotExist {
            tp: tp,
            name: subject.to_string(),
        })
    })
}

pub fn known_servers(tokens: &AppTokens) -> Result<Vec<(u64, String)>, Error> {
    let discord = try!(Discord::from_bot_token(&tokens.discord).map_err(|_| Error::LoginFailed("Discord")));

    discord.get_servers()
        .map_err(|_| {
            Error::Io {
                desc: "Discord servers",
                op: "list",
            }
        })
        .map(|servers| servers.into_iter().map(|s| (s.id.0, s.name)).collect())
}

pub fn get_valid_server<R: BufRead, W: Write>(servers: Vec<(u64, String)>, input: &mut R, output: &mut W) -> u64 {
    get_valid("Servers the bot is invited to", "The server to post the feed in", servers, input, output)
}

pub fn channels_in_server(tokens: &AppTokens, server_id: u64) -> Result<Vec<(u64, String)>, Error> {
    let discord = try!(Discord::from_bot_token(&tokens.discord).map_err(|_| Error::LoginFailed("Discord")));

    // TODO: uncomment once discord v0.9.0 gets released
    // discord.get_server_channels(ServerId(server_id))
    //     .map_err(|_| {
    //         Error::Io {
    //             desc: "Discord channels",
    //             op: "list",
    //         }
    //     })
    //     .map(|channels| channels.into_iter().map(|c| (c.id.0, c.name)).collect())

    Ok(vec![(0, "#general".to_string())])
}

pub fn get_valid_channel<R: BufRead, W: Write>(channels: Vec<(u64, String)>, input: &mut R, output: &mut W) -> u64 {
    get_valid("Channels in the chosen server", "The channel to post the feed in", channels, input, output)
}

fn get_valid<R: BufRead, W: Write>(list_heading: &str, prompt: &str, instances: Vec<(u64, String)>, input: &mut R, output: &mut W) -> u64 {
    writeln!(output, "{}:", list_heading).unwrap();
    for (idx, &(_, ref name)) in instances.iter().enumerate() {
        writeln!(output, "  {}. {}", idx + 1, name).unwrap();
    }
    writeln!(output, "").unwrap();

    let chosen = prompt_nonzero_len(input,
                                    output,
                                    prompt,
                                    |s| usize::from_str(&s).map(|idx| idx > 0 && idx <= instances.len()).unwrap_or(false))
        .unwrap();
    instances[usize::from_str(&chosen).unwrap() - 1].0
}
