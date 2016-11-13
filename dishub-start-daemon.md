dishub-start-daemon(1) -- Posting GitHub activity on Discord - posting
======================================================================

## SYNOPSIS

`dishub` [OPTIONS] `start-daemon` [START_DAEMON_OPTIONS]

## DESCRIPTION

Start the daemon that will queue and post events to Discord,

Requires tokens to be set, so be sure to run dishub-init(1) beforehand.

Will, of course, not post anything until you run dishub-add-feeds(1).

Loops forever and doesn't terminate but with a Ctrl-C equivalent.

For description of `dishub` itself see `dishub(1).

## OPTIONS

  See dishub(1).

## START_DAEMON_OPTIONS

  -s --sleep &lt;<sleep_time>&gt;

    Time to wait between checking for and posting events.

    Default: 60s. Format: NNs.

## EXAMPLES

  `dishub start-daemon`

    Successfully sent 14 events from nabijaczleweli
    Successfully sent 7 events from nabijaczleweli/dishub
    No new events in nabijaczleweli/cargo-update
    Too early to re-poll nabijaczleweli/safe-transmute-rs
    ...

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/dishub/issues>&gt;

## SEE ALSO

&lt;<https://github.com/nabijaczleweli/dishub>&gt;
