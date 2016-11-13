//! This module contains the functions used only by the `add_feeds` subsystem.
//!
//! The flow of the `add_feeds` subsystem is as follows:
//!
//! ```plaintext
//! Options::parse()
//! |> ops::add_feeds::verify()
//! // Read the subject
//! |> ops::add_feeds::get_watch_subject()
//! |> ops::AppTokens::read()
//! // Check if the subject exists
//! |> ops::add_feeds::verify_subject()
//! // Check which servers the bot's invited to and ask the user which one to post in.
//! |> ops::add_feeds::known_servers()
//! |> ops::add_feeds::get_valid_server()
//! // List channels in the specified server and ask the user which one to post in.
//! |> ops::add_feeds::channels_in_server()
//! |> ops::add_feeds::get_valid_channel()
//! // Update the feed list
//! |> ops::Feed::read()
//! |> ops::Feed::new()
//! |> ops::Feed::write()
//! ```


use self::super::{AppTokens, Feed, github, verify_file};
use self::super::super::util::prompt_nonzero_len;
use discord::model::{ChannelType, ServerId};
use self::super::super::Error;
use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::str::FromStr;
use discord::Discord;


/// Verify if, given the current configuration, it's permitted to continue with the subsequent steps of the `add_feeds`
/// subsystem.
///
/// The return value contains either the path to the file containing the global app tokens and the path to the file containing
/// the global feeds or why getting them failed.
///
/// The app tokens are required, but if the feeds file doesn't exist and empty one will be created.
///
/// # Examples
///
/// Verifying an existing tokens file.
///
/// ```
/// # use dishub::ops::add_feeds;
/// # use std::fs::{self, File};
/// # use std::env::temp_dir;
/// # use std::io::Write;
/// let tf = temp_dir().join("dishub-doctest").join("ops-add_feeds-verify-0");
/// fs::create_dir_all(&tf).unwrap();
/// File::create(tf.join("tokens.toml")).unwrap().write(&[]).unwrap();
///
/// assert_eq!(add_feeds::verify(&("$TEMP/ops-add_feeds-verify-0".to_string(), tf.clone())),
///            Ok((tf.join("tokens.toml"), tf.join("feeds.toml"))));
/// assert!(tf.join("feeds.toml").exists());
/// ```
///
/// Verifying a nonexistant tokens file.
///
/// ```
/// # use dishub::ops::add_feeds;
/// # use std::env::temp_dir;
/// # use dishub::Error;
/// let tf = temp_dir().join("dishub-doctest").join("ops-add_feeds-verify-1");
/// assert_eq!(add_feeds::verify(&("$TEMP/ops-add_feeds-verify-1".to_string(), tf)),
///            Err(Error::RequiredFileFromSubsystemNonexistant {
///                    subsys: "init",
///                    fname: "$TEMP/ops-add_feeds-verify-1/tokens.toml".to_string(),
///                }));
/// ```
pub fn verify(config_dir: &(String, PathBuf)) -> Result<(PathBuf, PathBuf), Error> {
    let tokens = try!(verify_file("tokens.toml", true, config_dir, false, "init"));
    let feeds = try!(verify_file("feeds.toml", true, config_dir, true, ""));

    if !feeds.exists() {
        Feed::write(vec![], &feeds);
    }

    Ok((tokens, feeds))
}

/// Prompt the user for the subject to watch.
///
/// This can be either a username (0 slashes) or a repo slug (1 slash), will reprompt if given more slashes.
///
/// # Examples
///
/// ```
/// # use dishub::ops::add_feeds;
/// # use std::io::BufReader;
/// assert_eq!(add_feeds::get_watch_subject(&mut BufReader::new(&b"nabijaczleweli\n"[..]),
///                                         &mut Vec::new()),
///            "nabijaczleweli".to_string());
/// assert_eq!(add_feeds::get_watch_subject(&mut BufReader::new(&b"nabijaczleweli/dishub\n"[..]),
///                                         &mut Vec::new()),
///            "nabijaczleweli/dishub".to_string());
///
/// assert_eq!(add_feeds::get_watch_subject(&mut BufReader::new(&b"nabijaczleweli/dishub/gargage\n\
///                                                               sehe\n"[..]), &mut Vec::new()),
///            "sehe".to_string());
/// ```
pub fn get_watch_subject<R: BufRead, W: Write>(input: &mut R, output: &mut W) -> String {
    prompt_nonzero_len(input,
                       output,
                       "What to watch (repo slug or user)",
                       |s| s.chars().filter(|&c| c == '/').count() <= 1)
        .unwrap()
}

/// Verify, whether the subject exists.
///
/// # Examples
///
/// Existing subjects.
///
/// ```
/// # use dishub::ops::{add_feeds, AppTokens};
/// # let tokens = AppTokens {
/// #     github: "".to_string(),
/// #     discord: "".to_string(),
/// # };
/// assert!(add_feeds::verify_subject("nabijaczleweli", &tokens).is_ok());
/// assert!(add_feeds::verify_subject("nabijaczleweli/dishub", &tokens).is_ok());
/// ```
///
/// Non-existant subject.
///
/// ```no_run
/// # use dishub::ops::{add_feeds, AppTokens};
/// # use dishub::Error;
/// # let tokens = AppTokens {
/// #     github: "".to_string(),
/// #     discord: "".to_string(),
/// # };
/// assert_eq!(add_feeds::verify_subject("3f0ada6056fe3fc/67a6682230bf1cb6d", &tokens),
///            Err(Error::WatchedDoesNotExist {
///                tp: "repository",
///                name: "3f0ada6056fe3fc/67a6682230bf1cb6d".to_string(),
///            }));
/// ```
pub fn verify_subject(subject: &str, tokens: &AppTokens) -> Result<(), Error> {
    type ExistCheck = fn(&str, &AppTokens) -> Result<bool, Error>;
    let (check_fn, tp): (ExistCheck, &str) = if subject.contains('/') {
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

/// Get all servers the bot is invited to.
///
/// This requires the passed `AppTokens` to have a valid Discord token.
///
/// The returned vector is of tuples (server ID, server display name).
///
/// # Examples
///
/// ```no_run
/// # use dishub::ops::{add_feeds, AppTokens};
/// # use dishub::Error;
/// # fn main() {
/// #     realmain().unwrap();
/// # }
/// # fn realmain() -> Result<(), Error> {
/// # let tokens = AppTokens {
/// #     github: "".to_string(),
/// #     discord: "".to_string(),
/// # };
/// let servers = try!(add_feeds::known_servers(&tokens));
/// for (id, name) in servers {
///     println!("{}: {}", id, name);
/// }
/// # Ok(())
/// # }
/// ```
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

/// Prompt the user to choose a server to post in, given a list of servers.
///
/// Get the server list from `known_servers()`.
///
/// The returned number is the chosen server's ID.
///
/// # Examples
///
/// ```
/// # use dishub::ops::add_feeds;
/// # use std::io::BufReader;
/// let servers = vec![(1000, "Babby's first server".to_string()),
///                    (1001, "Helo am server plz gib credentials".to_string())];
/// assert_eq!(add_feeds::get_valid_server(servers, &mut BufReader::new(&b"2\n"[..]), &mut Vec::new()),
///            1001);
/// ```
pub fn get_valid_server<R: BufRead, W: Write>(servers: Vec<(u64, String)>, input: &mut R, output: &mut W) -> u64 {
    get_valid("Servers the bot is invited to", "The server to post the feed in", servers, input, output)
}

/// List the channels in the specified server.
///
/// This requires the passed `AppTokens` to have a valid Discord token.
///
/// The returned vector is of tuples (channel ID, channel display name).
///
/// # Examples
///
/// ```no_run
/// # use dishub::ops::{add_feeds, AppTokens};
/// # use dishub::Error;
/// # fn main() {
/// #     realmain().unwrap();
/// # }
/// # fn realmain() -> Result<(), Error> {
/// # let tokens = AppTokens {
/// #     github: "".to_string(),
/// #     discord: "".to_string(),
/// # };
/// let server_id = 1001;
/// let channels = try!(add_feeds::channels_in_server(&tokens, 1001));
/// for (id, name) in channels {
///     println!("{}: {}", id, name);
/// }
/// # Ok(())
/// # }
/// ```
pub fn channels_in_server(tokens: &AppTokens, server_id: u64) -> Result<Vec<(u64, String)>, Error> {
    let discord = try!(Discord::from_bot_token(&tokens.discord).map_err(|_| Error::LoginFailed("Discord")));

    discord.get_server_channels(ServerId(server_id))
        .map_err(|_| {
            Error::Io {
                desc: "Discord channels",
                op: "list",
            }
        })
        .map(|channels| channels.into_iter().filter(|c| c.kind == ChannelType::Text).map(|c| (c.id.0, format!("#{}", c.name))).collect())
}

/// Prompt the user to choose a channel to post in, given a list of channels.
///
/// Get the channel list from `channels_in_server()`.
///
/// The returned number is the chosen channel's ID.
///
/// # Examples
///
/// ```
/// # use dishub::ops::add_feeds;
/// # use std::io::BufReader;
/// let servers = vec![(2000, "#general".to_string()),
///                    (2001, "#dishub-dump".to_string())];
/// assert_eq!(add_feeds::get_valid_channel(servers, &mut BufReader::new(&b"1\n"[..]), &mut Vec::new()),
///            2000);
/// ```
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
                                    |s| usize::from_str(s).map(|idx| idx > 0 && idx <= instances.len()).unwrap_or(false))
        .unwrap();
    instances[usize::from_str(&chosen).unwrap() - 1].0
}
