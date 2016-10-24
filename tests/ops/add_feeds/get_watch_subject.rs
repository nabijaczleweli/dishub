use dishub::ops::add_feeds::get_watch_subject;
use std::io::Cursor;


#[test]
fn user() {
    let mut out = Vec::new();
    let result = get_watch_subject(&mut Cursor::new(b"sehe\n"), &mut out);

    assert_eq!(&result, "sehe");
    assert_eq!(out[..], b"What to watch (repo slug or user): "[..]);
}

#[test]
fn repo_slug() {
    let mut out = Vec::new();
    let result = get_watch_subject(&mut Cursor::new(b"sehe/opus\n"), &mut out);

    assert_eq!(&result, "sehe/opus");
    assert_eq!(out[..], b"What to watch (repo slug or user): "[..]);
}

#[test]
fn three_slashes() {
    let mut out = Vec::new();
    let result = get_watch_subject(&mut Cursor::new(b"sehe/opus/issues\nsehe\n"), &mut out);

    assert_eq!(&result, "sehe");
    assert_eq!(out[..], b"What to watch (repo slug or user): What to watch (repo slug or user): "[..]);
}

#[test]
fn empty() {
    let mut out = Vec::new();
    let result = get_watch_subject(&mut Cursor::new(b"\nsehe\n"), &mut out);

    assert_eq!(&result, "sehe");
    assert_eq!(out[..], b"What to watch (repo slug or user): What to watch (repo slug or user): "[..]);
}
