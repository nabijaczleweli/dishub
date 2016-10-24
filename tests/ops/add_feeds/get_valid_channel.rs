use dishub::ops::add_feeds::get_valid_channel;
use std::io::Cursor;


#[test]
fn correct() {
    let mut out = Vec::new();
    let result =
        get_valid_channel(vec![(10, "#general".to_string()), (20, "#gaems".to_string()), (30, "#capitalists-dream".to_string()), (40, "#rust".to_string())],
                          &mut Cursor::new(b"3\n"),
                          &mut out);

    assert_eq!(result, 30);
    assert_eq!(&out[..],
               &b"Channels in the chosen server:\n\
                  \x20\x201. #general\n\
                  \x20\x202. #gaems\n\
                  \x20\x203. #capitalists-dream\n\
                  \x20\x204. #rust\n\
                  \n\
                  The channel to post the feed in: "[..]);
}

#[test]
fn oob() {
    let mut out = Vec::new();
    let result = get_valid_channel(vec![(10, "#general".to_string())], &mut Cursor::new(b"5\n1\n"), &mut out);

    assert_eq!(result, 10);
    assert_eq!(&out[..],
               &b"Channels in the chosen server:\n\
                  \x20\x201. #general\n\
                  \n\
                  The channel to post the feed in: The channel to post the feed in: "[..]);
}

#[test]
fn zero() {
    let mut out = Vec::new();
    let result = get_valid_channel(vec![(10, "#general".to_string())], &mut Cursor::new(b"0\n1\n"), &mut out);

    assert_eq!(result, 10);
    assert_eq!(&out[..],
               &b"Channels in the chosen server:\n\
                  \x20\x201. #general\n\
                  \n\
                  The channel to post the feed in: The channel to post the feed in: "[..]);
}
