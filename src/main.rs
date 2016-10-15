extern crate dishub;


use std::process::exit;
use std::io::{stderr, stdout, stdin};


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
    println!("{:#?}", opts);

    match opts.subsystem {
        dishub::options::Subsystem::Init { force } => init_main(opts, force),
    }
}

fn init_main(opts: dishub::options::Options, force: bool) -> Result<(), dishub::Error> {
    let data_path = try!(dishub::ops::init::verify(&opts.config_dir, force));

    let stdin = stdin();
    let mut lock = stdin.lock();

    let data = dishub::ops::init::get_data(&mut lock, &mut stdout());
    data.write(&data_path);

    println!("");
    println!("Remember to invite the bot to the servers you want it to post in!");

    Ok(())
}
