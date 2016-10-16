use self::super::super::make_dir;
use dishub::ops::init::verify;
use std::fs::File;
use dishub::Error;


#[test]
fn no_exist() {
    let path = make_dir("init-verify", "no_exist");
    let file_path = path.join("tokens.toml");

    assert_eq!(verify(&(String::new(), path), false), Ok(file_path));
}

#[test]
fn no_exist_force() {
    let path = make_dir("init-verify", "no_exist_force");
    let file_path = path.join("tokens.toml");

    assert_eq!(verify(&(String::new(), path), true), Ok(file_path));
}

#[test]
fn exist() {
    let path = make_dir("init-verify", "exist");
    let file_path = path.join("tokens.toml");

    File::create(&file_path).unwrap();

    assert_eq!(verify(&("TEST_FOLDER".to_string(), path), false),
               Err(Error::OverrideNoForce("TEST_FOLDER/tokens.toml".to_string())));
}

#[test]
fn exist_force() {
    let path = make_dir("init-verify", "exist_force");
    let file_path = path.join("tokens.toml");

    File::create(&file_path).unwrap();

    assert_eq!(verify(&(String::new(), path), true), Ok(file_path));
}
