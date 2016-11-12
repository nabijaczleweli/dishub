use self::super::{AppTokens, Event, read_toml_file, github};
use chrono::{FixedOffset, Duration, DateTime, Local};
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
    /// The minimal time the next poll is allowed.
    pub next_min: Option<DateTime<FixedOffset>>,
    /// Latest event's ID, this is required because GH API returns *all* events despite passing an ETag.
    pub latest_event: Option<u64>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct FeedForSerialisation {
    pub subject: String,

    pub server: u64,
    pub channel: u64,

    pub e_tag: Option<String>,
    pub latest: Option<String>,
    pub next_min: Option<String>,
    pub latest_event: Option<u64>,
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
            next_min: None,
            latest_event: None,
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

    pub fn poll(&mut self, tkn: &AppTokens) -> Result<Vec<Event>, Error> {
        let (mut events, next) = if self.e_tag.is_none() {
            let (ctnt, etag, next) = try!(if !self.subject.contains('/') {
                github::poll_user_events_new(&self.subject, tkn)
            } else {
                github::poll_repo_events_new(&self.subject, tkn)
            });

            self.e_tag = Some(etag);
            (Event::parse(&ctnt), next)
        } else {
            let (ctnt_etag, next) = try!(if !self.subject.contains('/') {
                github::poll_user_events_update(&self.subject, self.e_tag.as_ref().unwrap(), tkn)
            } else {
                github::poll_repo_events_update(&self.subject, self.e_tag.as_ref().unwrap(), tkn)
            });

            match ctnt_etag {
                Some((ctnt, etag)) => {
                    self.e_tag = Some(etag);
                    (Event::parse(&ctnt), next)
                }
                None => (vec![], next),
            }
        };

        let now = Local::now();
        let now = now.with_timezone(now.offset());

        self.latest = Some(now.clone());
        self.next_min = Some(now + Duration::seconds(next as i64));

        events.reverse();
        if let Some(latest_event_id) = self.latest_event {
            events = events.into_iter().skip_while(|ev| ev.id != latest_event_id).skip(1).collect();
        }
        if !events.is_empty() {
            self.latest_event = Some(events[events.len() - 1].id);
        }
        Ok(events)
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
            next_min: f.next_min.map(|dt| dt.to_rfc3339()),
            latest_event: f.latest_event,
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
            next_min: self.next_min.map(|dts| DateTime::parse_from_rfc3339(&dts).unwrap()),
            latest_event: self.latest_event,
        }
    }
}
