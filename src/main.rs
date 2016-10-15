extern crate dishub;


use std::io::stderr;
use std::process::exit;


fn main() {
    let result = actual_main();
    exit(result);
}

fn actual_main() -> i32 {
    if let Err(err) = result_main() {
        err.print_error(&mut stderr());
        err.exit_value()
    } else {
        0
    }
}

fn result_main() -> Result<(), dishub::Error> {
    let opts = dishub::options::Options::parse();
    print!("{:#?}", opts);

    Ok(())
}
