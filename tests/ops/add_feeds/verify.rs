use dishub::ops::add_feeds::verify;
use self::super::super::make_dir;
use std::fs::{File, remove_file};
use dishub::ops::Feed;
use dishub::Error;


#[test]
fn tokens_no_exist_feeds_no_exist() {
    let path = make_dir("add_feeds-verify", "tokens_no_exist_feeds_no_exist");
    let feeds_path = path.join("feeds.toml");

    assert_eq!(verify(&("TEST_FOLDER".to_string(), path)),
               Err(Error::RequiredFileFromSubsystemNonexistant {
                   subsys: "init",
                   fname: "TEST_FOLDER/tokens.toml".to_string(),
               }));

    assert!(!feeds_path.exists());
}

#[test]
fn tokens_no_exist_feeds_exist() {
    let path = make_dir("add_feeds-verify", "tokens_no_exist_feeds_exist");
    let feeds_path = path.join("feeds.toml");

    File::create(&feeds_path).unwrap();

    assert_eq!(verify(&("TEST_FOLDER".to_string(), path)),
               Err(Error::RequiredFileFromSubsystemNonexistant {
                   subsys: "init",
                   fname: "TEST_FOLDER/tokens.toml".to_string(),
               }));
}

#[test]
fn tokens_exist_feeds_no_exist() {
    let path = make_dir("add_feeds-verify", "tokens_exist_feeds_no_exist");
    let tokens_path = path.join("tokens.toml");
    let feeds_path = path.join("feeds.toml");

    File::create(&tokens_path).unwrap();

    assert_eq!(verify(&(String::new(), path)), Ok((tokens_path, feeds_path.clone())));

    assert!(feeds_path.exists());
    assert_eq!(Feed::read(&feeds_path), Ok(vec![]));
    remove_file(feeds_path).unwrap();
}

#[test]
fn tokens_exist_feeds_exist() {
    let path = make_dir("add_feeds-verify", "tokens_exist_feeds_exist");
    let tokens_path = path.join("tokens.toml");
    let feeds_path = path.join("feeds.toml");

    File::create(&tokens_path).unwrap();
    File::create(&feeds_path).unwrap();

    assert_eq!(verify(&(String::new(), path)), Ok((tokens_path, feeds_path)));
}
