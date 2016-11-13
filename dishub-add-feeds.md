dishub-add-feeds(1) -- Posting GitHub activity on Discord - following
=====================================================================

## SYNOPSIS

`dishub` [OPTIONS] `add-feeds`

## DESCRIPTION

Follow a user or a repository and
specify the channel and server to post the events to.

Requires tokens to be set, so be sure to run dishub-init(1) beforehand.

For description of `dishub` itself see dishub(1).

## OPTIONS

  See dishub(1).

## EXAMPLES

  `dishub add-feeds`

    Existing user/repository and tokens set:

      What to watch (repo slug or user): nabijaczleweli/dishub
      Servers the bot is invited to:
        1. Dishub Test

      The server to post the feed in: 1
      Channels in the chosen server:
        1. #all
        2. #dishub
        3. #cargo-update
        4. #safe-transmute

      The channel to post the feed in: 2

  `dishub add-feeds`

    Non-existant user/repository but tokens set:

      What to watch (repo slug or user): 3f0ada6056fe3fc67a6682230bf1cb6d
      The watched user "3f0ada6056fe3fc67a6682230bf1cb6d" doesn't exist.

  `dishub add-feeds`

    Tokens not set.

      Run the init subsystem first to produce "$HOME/.dishub/tokens.toml".

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/dishub/issues>&gt;

## SEE ALSO

&lt;<https://github.com/nabijaczleweli/dishub>&gt;
