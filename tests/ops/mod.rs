use std::fs::create_dir;
use std::path::PathBuf;
use std::env::temp_dir;

mod init;
mod feed;
mod app_tokens;
mod add_feeds;
mod unfollow_feeds;


fn make_dir(section: &str, function: &str) -> PathBuf {
    let mut tf = temp_dir();
    let _ = create_dir(&tf);
    tf.push("dishub-test");
    let _ = create_dir(&tf);
    tf.push(format!("ops-{}-{}", section, function));
    let _ = create_dir(&tf);
    tf
}
