use self::super::{AppTokens, Event, Feed, verify_file};
use discord::{Discord, Error as DisErr};
use discord::model::ChannelId;
use self::super::super::Error;
use chrono::{Duration, Local};
use std::path::PathBuf;
use std::io::Write;
use std::thread;


pub fn verify(config_dir: &(String, PathBuf)) -> Result<(PathBuf, PathBuf), Error> {
    let tokens = try!(verify_file("tokens.toml", true, config_dir, false, "init"));
    let feeds = try!(verify_file("feeds.toml", true, config_dir, false, "add-feeds"));

    Ok((tokens, feeds))
}

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

pub fn post_text(ev: &Event) -> String {
    let urls = ev.urls();
    let mut txt = Vec::new();

    write!(txt, "{}", ev).unwrap();
    if !urls.is_empty() {
        for url in urls {
            writeln!(txt, "").unwrap();
            write!(txt, "<{}>", url).unwrap();
        }
    }

    String::from_utf8(txt).unwrap()
}

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
