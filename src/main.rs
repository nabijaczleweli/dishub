extern crate dishub;

use std::thread;
use std::process::exit;
use std::time::Duration;
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
        dishub::options::Subsystem::AddFeeds => add_feeds_main(opts),
        dishub::options::Subsystem::UnfollowFeeds => unfollow_feeds_main(opts),
        dishub::options::Subsystem::StartDaemon { sleep } => start_daemon_main(opts, sleep),
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

fn add_feeds_main(opts: dishub::options::Options) -> Result<(), dishub::Error> {
    let (data_path, feeds_path) = try!(dishub::ops::add_feeds::verify(&opts.config_dir));

    let stdin = stdin();
    let mut lock = stdin.lock();

    let subject = dishub::ops::add_feeds::get_watch_subject(&mut lock, &mut stdout());
    let tokens = try!(dishub::ops::AppTokens::read(&data_path));

    try!(dishub::ops::add_feeds::verify_subject(&subject, &tokens));

    let servers = try!(dishub::ops::add_feeds::known_servers(&tokens));
    let server = dishub::ops::add_feeds::get_valid_server(servers, &mut lock, &mut stdout());

    let channels = try!(dishub::ops::add_feeds::channels_in_server(&tokens, server));
    let channel = dishub::ops::add_feeds::get_valid_channel(channels, &mut lock, &mut stdout());

    let mut feeds = try!(dishub::ops::Feed::read(&feeds_path));
    feeds.push(dishub::ops::Feed::new(subject, server, channel));
    dishub::ops::Feed::write(feeds, &feeds_path);

    Ok(())
}

fn unfollow_feeds_main(opts: dishub::options::Options) -> Result<(), dishub::Error> {
    let feeds_path = try!(dishub::ops::unfollow_feeds::verify(&opts.config_dir));

    let stdin = stdin();
    let mut lock = stdin.lock();

    let mut feeds = try!(dishub::ops::Feed::read(&feeds_path));
    dishub::ops::unfollow_feeds::print_feeds(&feeds, &mut stdout());
    let to_remove = dishub::ops::unfollow_feeds::get_feeds_to_remove(&feeds, &mut lock, &mut stdout());

    let idx_to_remove: Vec<_> = feeds.iter().enumerate().filter(|&(_, f)| to_remove.contains(&f.subject)).map(|(i, _)| i).collect();
    for idx in idx_to_remove {
        feeds.remove(idx);
    }

    dishub::ops::Feed::write(feeds, &feeds_path);

    Ok(())
}

fn start_daemon_main(opts: dishub::options::Options, sleep: Duration) -> Result<(), dishub::Error> {
    let (tokens_path, feeds_path) = try!(dishub::ops::start_daemon::verify(&opts.config_dir));
    let tokens = try!(dishub::ops::AppTokens::read(&tokens_path));

    loop {
        let mut feeds = try!(dishub::ops::Feed::read(&feeds_path));

        for feed in feeds.iter_mut().filter(|f| dishub::ops::start_daemon::feeds_filter(&mut stdout(), f)) {
            let events = try!(feed.poll(&tokens));
            if events.is_empty() {
                println!("No new events in {}", feed.subject);
            } else {
                try!(dishub::ops::start_daemon::send_messages(&tokens, events.iter().map(dishub::ops::start_daemon::post_text).collect(), feed.channel));
                println!("Successfully sent {} event{} from {}",
                         events.len(),
                         if events.len() != 1 { "s" } else { "" },
                         feed.subject);
            }
        }

        dishub::ops::Feed::write(feeds, &feeds_path);
        thread::sleep(sleep);
    }
}
