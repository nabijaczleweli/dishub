use self::super::{Feed, verify_file};
use self::super::super::Error;
use std::path::PathBuf;
use std::io::Write;
use chrono::Local;


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
