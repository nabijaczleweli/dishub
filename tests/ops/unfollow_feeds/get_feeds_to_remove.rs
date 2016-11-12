use dishub::ops::unfollow_feeds::get_feeds_to_remove;
use dishub::util::mul_str;
use self::super::feed;
use std::io::Cursor;


#[test]
fn empty() {
    let mut out = Vec::new();
    let result = get_feeds_to_remove(&[feed("nabijaczleweli"), feed("sehe/opus")], &mut Cursor::new(b"\n"), &mut out);

    assert!(result.is_empty());
    assert_eq!(&out[..], &b"The feed to unsubscribe from (or empty to end): "[..]);
}

#[test]
fn single() {
    let mut out = Vec::new();
    let result = get_feeds_to_remove(&[feed("nabijaczleweli"), feed("sehe/opus")], &mut Cursor::new(b"nabijaczleweli\n\n"), &mut out);

    assert_eq!(result, vec!["nabijaczleweli".to_string()]);
    assert_eq!(&out[..],
               &b"The feed to unsubscribe from (or empty to end): The feed to unsubscribe from (or empty to end): "[..]);
}

#[test]
fn multi() {
    let mut out = Vec::new();
    let result = get_feeds_to_remove(&[feed("nabijaczleweli"), feed("sehe/opus"), feed("shepmaster/jetscii")],
                                     &mut Cursor::new(&b"nabijaczleweli\nsehe/opus\nshepmaster/jetscii\n\n"[..]),
                                     &mut out);

    assert_eq!(result,
               vec!["nabijaczleweli".to_string(), "sehe/opus".to_string(), "shepmaster/jetscii".to_string()]);
    assert_eq!(&out[..], mul_str("The feed to unsubscribe from (or empty to end): ", 4).as_bytes());
}
