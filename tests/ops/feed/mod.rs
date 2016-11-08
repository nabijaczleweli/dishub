use dishub::ops::Feed;

mod write;
mod read;


#[test]
fn new() {
    assert_eq!(Feed::new("nabijaczleweli".to_string(), 105, 1056),
               Feed {
                   subject: "nabijaczleweli".to_string(),
                   server: 105,
                   channel: 1056,
                   e_tag: None,
                   latest: None,
                   next_min: None,
               });
}
