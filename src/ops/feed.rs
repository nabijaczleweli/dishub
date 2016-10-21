use chrono::{DateTime, FixedOffset};
use self::super::read_toml_file;
use self::super::super::Error;
use toml::encode_str;
use std::path::Path;
use std::io::Write;
use std::fs::File;


/// A feed to be checked on GitHub and sent to Discord.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Feed {
    /// The thing to watch.
    ///
    /// This can either be a user in the form: `"username"` or a repo slug in the form: `"username/reponame"`.
    pub subject: String,

    /// The Discord server ID to post to.
    pub server: u64,
    /// The Discord channel ID to post to.
    pub channel: u64,

    /// The ETag of the last received event pack.
    pub e_tag: Option<String>,
    /// The time of the last received event pack.
    pub latest: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct FeedForSerialisation {
    pub subject: String,

    pub server: u64,
    pub channel: u64,

    pub e_tag: Option<String>,
    pub latest: Option<String>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, RustcEncodable, RustcDecodable)]
struct Feeds {
    feed: Vec<FeedForSerialisation>,
}

impl Feed {
    /// Create an unpolled feed with the specified subject and posting target.
    pub fn new(subject: String, server_id: u64, channel_id: u64) -> Feed {
        Feed {
            subject: subject,
            server: server_id,
            channel: channel_id,
            e_tag: None,
            latest: None,
        }
    }

    /// Read the application feeds from the specified file.
    pub fn read(p: &Path) -> Result<Vec<Feed>, Error> {
        let feeds: Feeds = try!(read_toml_file(p, "Followed feeds"));
        Ok(feeds.feed.into_iter().map(FeedForSerialisation::into).collect())
    }

    /// Save the application feeds to the specified file.
    pub fn write(feeds: Vec<Feed>, p: &Path) {
        File::create(p).unwrap().write_all(encode_str(&Feeds { feed: feeds.into_iter().map(FeedForSerialisation::from).collect() }).as_bytes()).unwrap();
    }
}

impl From<Feed> for FeedForSerialisation {
    fn from(f: Feed) -> FeedForSerialisation {
        FeedForSerialisation {
            subject: f.subject,
            server: f.server,
            channel: f.channel,
            e_tag: f.e_tag,
            latest: f.latest.map(|dt| dt.to_rfc3339()),
        }
    }
}

impl Into<Feed> for FeedForSerialisation {
    fn into(self) -> Feed {
        Feed {
            subject: self.subject,
            server: self.server,
            channel: self.channel,
            e_tag: self.e_tag,
            latest: self.latest.map(|dts| DateTime::parse_from_rfc3339(&dts).unwrap()),
        }
    }
}
