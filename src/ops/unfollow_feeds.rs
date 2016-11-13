//! This module contains the functions used only by the `unfollow_feeds` subsystem.
//!
//! The flow of the `unfollow_feeds` subsystem is as follows:
//!
//! ```plaintext
//! Options::parse()
//! |> ops::unfollow_feeds::verify()
//! |> ops::Feed::read()
//! |> ops::unfollow_feeds::print_feeds()
//! |> ops::unfollow_feeds::get_feeds_to_remove()
//! |> ops::Feed::write()
//! ```


use self::super::super::util::prompt_any_len;
use self::super::{Feed, verify_file};
use self::super::super::Error;
use std::io::{BufRead, Write};
use std::path::PathBuf;


/// Verify if, given the current configuration, it's permitted to continue with the subsequent steps of the `unfollow_feeds`
/// subsystem.
///
/// The return value contains either the path to the path to the file containing the global feeds or why getting it failed.
///
/// # Examples
///
/// Verifying an existing feeds file.
///
/// ```
/// # use dishub::ops::unfollow_feeds;
/// # use std::fs::{self, File};
/// # use std::env::temp_dir;
/// # use std::io::Write;
/// let tf = temp_dir().join("dishub-doctest").join("ops-unfollow_feeds-verify-0");
/// fs::create_dir_all(&tf).unwrap();
/// File::create(tf.join("feeds.toml")).unwrap().write(&[]).unwrap();
///
/// assert_eq!(unfollow_feeds::verify(&("$TEMP/ops-unfollow_feeds-verify-0".to_string(), tf.clone())),
///            Ok(tf.join("feeds.toml")));
/// assert!(tf.join("feeds.toml").exists());
/// ```
///
/// Verifying a nonexistant feeds file.
///
/// ```
/// # use dishub::ops::unfollow_feeds;
/// # use std::env::temp_dir;
/// # use dishub::Error;
/// let tf = temp_dir().join("dishub-doctest").join("ops-unfollow_feeds-verify-1");
/// assert_eq!(unfollow_feeds::verify(&("$TEMP/ops-unfollow_feeds-verify-1".to_string(), tf)),
///            Err(Error::RequiredFileFromSubsystemNonexistant {
///                    subsys: "add-feeds",
///                    fname: "$TEMP/ops-unfollow_feeds-verify-1/feeds.toml".to_string(),
///                }));
/// ```
pub fn verify(config_dir: &(String, PathBuf)) -> Result<PathBuf, Error> {
    verify_file("feeds.toml", true, config_dir, false, "add-feeds")
}

/// Print the subscribed-to feeds in a human-readable format.
///
/// # Examples
///
/// ```
/// # use dishub::ops::{unfollow_feeds, Feed};
/// let mut out = Vec::new();
/// unfollow_feeds::print_feeds(&[Feed::new("nabijaczleweli".to_string(), 0, 0),
///                               Feed::new("nabijaczleweli/dishub".to_string(), 0, 1)],
///                             &mut out);
/// assert_eq!(&out[..],
///            &b"The feeds currently subscribed to:\n\
///               \x20\x20nabijaczleweli\n\
///               \x20\x20nabijaczleweli/dishub\n\
///               \n"[..]);
/// ```
pub fn print_feeds<W: Write>(feeds: &[Feed], output: &mut W) {
    writeln!(output, "The feeds currently subscribed to:").unwrap();
    for feed in feeds {
        writeln!(output, "  {}", feed.subject).unwrap();
    }
    writeln!(output, "").unwrap();
}

/// Prompt the user for the feed subjects to sunsubscribe from.
///
/// Will accept only subjects that exist in the supplied feeds.
///
/// # Examples
///
/// ```
/// # use dishub::ops::{unfollow_feeds, Feed};
/// # use std::io::BufReader;
/// assert_eq!(unfollow_feeds::get_feeds_to_remove(
///         &[Feed::new("nabijaczleweli".to_string(), 10, 0),
///           Feed::new("nabijaczleweli/dishub".to_string(), 11, 1),
///           Feed::new("sehe".to_string(), 12, 2)],
///         &mut BufReader::new(&b"nabijaczleweli\n\
///                                nabijaczleweli/dishub\n"[..]),
///         &mut Vec::new()),
///     vec!["nabijaczleweli".to_string(), "nabijaczleweli/dishub".to_string()]);
/// ```
pub fn get_feeds_to_remove<R: BufRead, W: Write>(feeds: &[Feed], input: &mut R, output: &mut W) -> Vec<String> {
    let mut selected_feeds = vec![];
    while let Some(s) = prompt_any_len(input,
                                       output,
                                       "The feed to unsubscribe from (or empty to end)",
                                       |s| feeds.iter().any(|f| f.subject == *s))
        .unwrap() {
        selected_feeds.push(s);
    }
    selected_feeds
}
