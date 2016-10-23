use dishub::ops::unfollow_feeds::verify;
use self::super::super::make_dir;
use std::fs::File;
use dishub::Error;


#[test]
fn no_exist() {
    let path = make_dir("unfollow_feeds-verify", "no_exist");

    assert_eq!(verify(&("TEST_FOLDER".to_string(), path)),
               Err(Error::RequiredFileFromSubsystemNonexistant {
                   subsys: "add-feeds",
                   fname: "TEST_FOLDER/feeds.toml".to_string(),
               }));
}

#[test]
fn exist() {
    let path = make_dir("unfollow_feeds-verify", "exist");
    let file_path = path.join("feeds.toml");

    File::create(&file_path).unwrap();

    assert_eq!(verify(&(String::new(), path)), Ok(file_path));
}
