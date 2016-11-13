dishub(1) -- Posting GitHub activity on Discord
===============================================

## SYNOPSIS

`dishub` [OPTIONS] &lt;SUBSYSTEM&gt; [SUBSYSTEM_OPTIONS]

## DESCRIPTION

Dishub is an app for posting GitHub activity on Discord.

See the subcommands for more information:

  * dishub-init(1) - authorising the application
  * dishub-add-feeds(1) - following people and repositories
  * dishub-unfollow-feeds(1) - following people and repositories
  * dishub-start-daemon(1) - start the tweet-posting daemon

## OPTIONS

  -c --config-dir &lt;<config_dir>&gt;

    Directory with the configuration.

    The configuration directory contains all of dishub's data.

    Default: $HOME/.dishub

## EXAMPLES

  See the per-subcommand examples page

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/dishub/issues>&gt;

## SEE ALSO

&lt;<https://github.com/nabijaczleweli/dishub>&gt;
