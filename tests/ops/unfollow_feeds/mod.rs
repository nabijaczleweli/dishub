use dishub::ops::unfollow_feeds;
use dishub::ops::Feed;

mod verify;
mod get_feeds_to_remove;


#[test]
fn print_feeds() {
    let mut out = Vec::new();
    unfollow_feeds::print_feeds(&[feed("nabijaczleweli"), feed("sehe/opus")], &mut out);

    assert_eq!(&out[..],
               &b"The feeds currently subscribed to:\n\
                  \x20\x20nabijaczleweli\n\
                  \x20\x20sehe/opus\n\
                  \n"[..]);
}

fn feed(subject: &str) -> Feed {
    Feed::new(subject.to_string(), 0, 0)
}
