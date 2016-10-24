use dishub::ops::add_feeds::get_valid_server;
use std::io::Cursor;
use std::str;


#[test]
fn correct() {
    let mut out = Vec::new();
    let result = get_valid_server(vec![(10, "Server 1".to_string()),
                                       (20, "Server 2".to_string()),
                                       (30, "Server".to_string()),
                                       (40, "Server with маджишян's letters".to_string())],
                                  &mut Cursor::new(b"3\n"),
                                  &mut out);

    assert_eq!(result, 30);
    assert_eq!(str::from_utf8(&out[..]).unwrap(),
               "Servers the bot is invited to:\n\
                \x20\x201. Server 1\n\
                \x20\x202. Server 2\n\
                \x20\x203. Server\n\
                \x20\x204. Server with маджишян's letters\n\
                \n\
                The server to post the feed in: ");
}

#[test]
fn oob() {
    let mut out = Vec::new();
    let result = get_valid_server(vec![(10, "Server 1".to_string())], &mut Cursor::new(b"5\n1\n"), &mut out);

    assert_eq!(result, 10);
    assert_eq!(str::from_utf8(&out[..]).unwrap(),
               "Servers the bot is invited to:\n\
                \x20\x201. Server 1\n\
                \n\
                The server to post the feed in: The server to post the feed in: ");
}

#[test]
fn zero() {
    let mut out = Vec::new();
    let result = get_valid_server(vec![(10, "Server 1".to_string())], &mut Cursor::new(b"0\n1\n"), &mut out);

    assert_eq!(result, 10);
    assert_eq!(str::from_utf8(&out[..]).unwrap(),
               "Servers the bot is invited to:\n\
                \x20\x201. Server 1\n\
                \n\
                The server to post the feed in: The server to post the feed in: ");
}
