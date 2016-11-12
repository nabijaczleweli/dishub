use self::super::super::util::prompt_any_len;
use self::super::{Feed, verify_file};
use self::super::super::Error;
use std::io::{BufRead, Write};
use std::path::PathBuf;


pub fn verify(config_dir: &(String, PathBuf)) -> Result<PathBuf, Error> {
    verify_file("feeds.toml", true, config_dir, false, "add-feeds")
}

pub fn print_feeds<W: Write>(feeds: &[Feed], output: &mut W) {
    writeln!(output, "The feeds currently subscribed to:").unwrap();
    for feed in feeds {
        writeln!(output, "  {}", feed.subject).unwrap();
    }
    writeln!(output, "").unwrap();
}

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
