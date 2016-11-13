//! This module contains the functions used only by the `start_daemon` subsystem.
//!
//! The flow of the `start_daemon` subsystem is as follows:
//!
//! ```plaintext
//! Options::parse()
//! |> ops::start_daemon::verify()
//! |> ops::AppTokens::read()
//! ```
//!
//! Then, repeatedly
//!
//! ```plaintext
//! ops::Feed::read()
//! |> ops::start_daemon::feeds_filter()
//! |> ops::Feed::poll()
//! |> ops::start_daemon::post_text()
//! |> ops::start_daemon::send_messages()
//! |> ops::Feed::write()
//! ```


use self::super::{AppTokens, Event, Feed, verify_file};
use discord::{Discord, Error as DisErr};
use discord::model::ChannelId;
use self::super::super::Error;
use chrono::{Duration, Local};
use std::path::PathBuf;
use std::io::Write;
use std::thread;


/// Verify if, given the current configuration, it's permitted to continue with the subsequent steps of the `start_daemon`
/// subsystem.
///
/// The return value contains either the path to the file containing the global app tokens and the path to the file containing
/// the global feeds or why getting them failed.
///
/// # Examples
///
/// Verifying an existing tokens and feeds file.
///
/// ```
/// # use dishub::ops::start_daemon;
/// # use std::fs::{self, File};
/// # use std::env::temp_dir;
/// # use std::io::Write;
/// let tf = temp_dir().join("dishub-doctest").join("ops-start_daemon-verify-0");
/// fs::create_dir_all(&tf).unwrap();
/// File::create(tf.join("tokens.toml")).unwrap().write(&[]).unwrap();
/// File::create(tf.join("feeds.toml")).unwrap().write(&[]).unwrap();
///
/// assert_eq!(start_daemon::verify(&("$TEMP/ops-start_daemon-verify-0".to_string(), tf.clone())),
///            Ok((tf.join("tokens.toml"), tf.join("feeds.toml"))));
/// ```
///
/// Verifying a nonexistant tokens file.
///
/// ```
/// # use dishub::ops::start_daemon;
/// # use std::env::temp_dir;
/// # use dishub::Error;
/// let tf = temp_dir().join("dishub-doctest").join("ops-start_daemon-verify-1");
/// assert_eq!(start_daemon::verify(&("$TEMP/ops-start_daemon-verify-1".to_string(), tf)),
///            Err(Error::RequiredFileFromSubsystemNonexistant {
///                    subsys: "init",
///                    fname: "$TEMP/ops-start_daemon-verify-1/tokens.toml".to_string(),
///                }));
/// ```
///
/// Verifying a nonexistant feeds file.
///
/// ```
/// # use dishub::ops::start_daemon;
/// # use std::fs::{self, File};
/// # use std::env::temp_dir;
/// # use std::io::Write;
/// # use dishub::Error;
/// let tf = temp_dir().join("dishub-doctest").join("ops-start_daemon-verify-2");
/// fs::create_dir_all(&tf).unwrap();
/// File::create(tf.join("tokens.toml")).unwrap().write(&[]).unwrap();
///
/// let tf = temp_dir().join("dishub-doctest").join("ops-start_daemon-verify-2");
/// assert_eq!(start_daemon::verify(&("$TEMP/ops-start_daemon-verify-2".to_string(), tf)),
///            Err(Error::RequiredFileFromSubsystemNonexistant {
///                    subsys: "add-feeds",
///                    fname: "$TEMP/ops-start_daemon-verify-2/feeds.toml".to_string(),
///                }));
/// ```
pub fn verify(config_dir: &(String, PathBuf)) -> Result<(PathBuf, PathBuf), Error> {
    let tokens = try!(verify_file("tokens.toml", true, config_dir, false, "init"));
    let feeds = try!(verify_file("feeds.toml", true, config_dir, false, "add-feeds"));

    Ok((tokens, feeds))
}

/// A filter function to use upon feeds to be polled.
///
/// # Examples
///
/// ```
/// # extern crate chrono;
/// # extern crate dishub;
/// # use chrono::{Duration, Local};
/// # use dishub::ops::{start_daemon, Feed};
/// # fn main() {
/// let mut out = Vec::new();
/// let now = Local::now();
/// let now = now.with_timezone(now.offset());
///
/// let feeds = vec![Feed::new("nabijaczleweli".to_string(), 10, 0),
///                  Feed {
///                      subject: "nabijaczleweli/dishub".to_string(),
///                      server: 11,
///                      channel: 1,
///                      e_tag: Some("4797f0ad2ee145181045fe69c61676e6".to_string()),
///                      latest: Some(now),
///                      next_min: Some(now + Duration::minutes(1)),
///                      latest_event: Some(4831774905),
///                  },
///                  Feed {
///                      subject: "sehe".to_string(),
///                      server: 12,
///                      channel: 2,
///                      e_tag: Some("62476f13306db1cfade222d41bcdcb51".to_string()),
///                      latest: Some(now - Duration::minutes(2)),
///                      next_min: Some(now - Duration::minutes(1)),
///                      latest_event: Some(4856265369),
///                  }];
///
/// let new_feeds: Vec<_> = feeds.iter().filter(|f| start_daemon::feeds_filter(&mut out, f)).collect();
///
/// assert_eq!(&out[..], &b"Too early to re-poll nabijaczleweli/dishub.\n"[..]);
/// assert_eq!(&new_feeds[..], &[&feeds[0], &feeds[2]]);
/// # }
/// ```
pub fn feeds_filter<W: Write>(output: &mut W, f: &Feed) -> bool {
    let now = Local::now();
    let now = now.with_timezone(now.offset());

    if f.next_min.is_some() && *f.next_min.as_ref().unwrap() > now {
        writeln!(output, "Too early to re-poll {}.", f.subject).unwrap();
        false
    } else {
        true
    }
}

/// Create a Discord message body from an event.
///
/// # Examples
///
/// ```
/// # extern crate chrono;
/// # extern crate dishub;
/// # use chrono::DateTime;
/// # use dishub::ops::{start_daemon, EventPayload, Event};
/// # fn main() {
/// assert_eq!(&start_daemon::post_text(&Event {
///     created_at: DateTime::parse_from_rfc2822("Tue, 8 Nov 2016 03:10:26 +0000").unwrap(),
///     actor: "carllhw".to_string(),
///     repo: "nabijaczleweli/cargo-update".to_string(),
///     id: 4831774905,
///     payload: EventPayload::Watch {
///         action: "started".to_string(),
///     },
/// }), "08.11.2016 03:10:26 AM: carllhw starred nabijaczleweli/cargo-update\n\
///      <https://github.com/nabijaczleweli/cargo-update/stargazers>");
/// # }
/// ```
pub fn post_text(ev: &Event) -> String {
    ev.urls().into_iter().fold(ev.to_string(), |t, u| t + "\n<" + &u + ">")
}

/// Post the specified messages to a Discord channel.
///
/// Will automatially wait on rate-limits.
///
/// # Examples
///
/// ```no_run
/// # extern crate chrono;
/// # extern crate dishub;
/// # use dishub::Error;
/// # use dishub::ops::{start_daemon, AppTokens};
/// # struct Feed {
/// #     channel: u64,
/// # }
/// # fn main() {
/// # let tokens = AppTokens {
/// #     github: "".to_string(),
/// #     discord: "".to_string(),
/// # };
/// # let feed = Feed {
/// #     channel: 10,
/// # };
/// start_daemon::send_messages(&tokens,
///     vec!["08.11.2016 03:10:26 AM: carllhw starred nabijaczleweli/cargo-update\n\
///           <https://github.com/nabijaczleweli/cargo_update/stargazers>".to_string(),
///          "09.11.2016 06:14:26 PM: sehe pushed 1 commit to sehe/opus\n\
///           <https://github.com/sehe/opus/compare/95659cd...eb282d9>".to_string()],
///     feed.channel).unwrap();
/// # }
/// ```
pub fn send_messages(tokens: &AppTokens, txts: Vec<String>, channel: u64) -> Result<(), Error> {
    let discord = try!(Discord::from_bot_token(&tokens.discord).map_err(|_| Error::LoginFailed("Discord")));

    for txt in txts {
        loop {
            match discord.send_message(&ChannelId(channel), &txt, "", false) {
                Err(DisErr::RateLimited(ms)) => thread::sleep(Duration::milliseconds(ms as i64).to_std().unwrap()),
                Err(_) => {
                    return Err(Error::Io {
                        desc: "event message",
                        op: "post",
                    })
                }
                Ok(_) => break,
            }
        }
    }

    Ok(())
}
