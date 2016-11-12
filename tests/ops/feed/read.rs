use self::super::super::make_dir;
use dishub::ops::Feed;
use chrono::DateTime;
use std::io::Write;
use std::fs::File;


#[test]
fn empty() {
    let mut path = make_dir("feed-read", "empty");
    path.push("feeds.toml");

    File::create(&path).unwrap().write_all(b"feed = []\n").unwrap();

    assert_eq!(Feed::read(&path), Ok(vec![]));
}

#[test]
fn unpolled() {
    let mut path = make_dir("feed-read", "unpolled");
    path.push("feeds.toml");

    File::create(&path)
        .unwrap()
        .write_all(b"[[feed]]\n\
                     channel = 105\n\
                     server = 1056\n\
                     subject = \"nabijaczleweli\"\n")
        .unwrap();

    assert_eq!(Feed::read(&path), Ok(vec![Feed::new("nabijaczleweli".to_string(), 1056, 105)]));
}

#[test]
fn polled() {
    let mut path = make_dir("feed-read", "polled");
    path.push("feeds.toml");

    File::create(&path)
        .unwrap()
        .write_all(b"[[feed]]\n\
                    channel = 105\n\
                    e_tag = \"a18c3bded88eb5dbb5c849a489412bf3\"\n\
                    latest = \"2000-07-01T15:12:57-08:00\"\n\
                    latest_event = 512\n\
                    next_min = \"2000-07-01T15:14:00-08:00\"\n\
                    server = 1056\n\
                    subject = \"nabijaczleweli\"\n")
        .unwrap();

    assert_eq!(Feed::read(&path),
               Ok(vec![Feed {
                           subject: "nabijaczleweli".to_string(),
                           server: 1056,
                           channel: 105,
                           e_tag: Some("a18c3bded88eb5dbb5c849a489412bf3".to_string()),
                           latest: Some(DateTime::parse_from_rfc2822("Sat, 1 Jul 2000 15:12:57 -0800").unwrap()),
                           next_min: Some(DateTime::parse_from_rfc2822("Sat, 1 Jul 2000 15:14:00 -0800").unwrap()),
                           latest_event: Some(512),
                       }]));
}
