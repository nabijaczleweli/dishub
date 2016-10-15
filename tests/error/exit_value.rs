use dishub::Error;


#[test]
fn override_no_force() {
    assert_eq!(Error::OverrideNoForce("".to_string()).exit_value(), 1);
}

#[test]
fn required_file_from_subsystem_nonexistant() {
    assert_eq!(Error::RequiredFileFromSubsystemNonexistant {
                       subsys: "",
                       fname: "".to_string(),
                   }
                   .exit_value(),
               2);
}

#[test]
fn file_parsing_failed() {
    assert_eq!(Error::FileParsingFailed {
                       desc: "",
                       errors: vec![],
                   }
                   .exit_value(),
               3);
}

#[test]
fn io() {
    assert_eq!(Error::Io { desc: "", op: "" }.exit_value(), 4);
}
