dishub-init(1) -- Posting GitHub activity on Discord - authorisation
====================================================================

## SYNOPSIS

`dishub` [OPTIONS] `init` [INIT_OPTIONS]

## DESCRIPTION

Set the GitHub and Discord tokens to use to authorise to the services.

A discord bot token can be acquired from
https://discordapp.com/developers/applications/me.

For description of `dishub` itself see dishub(1).

## OPTIONS

  See dishub(1).

## INIT_OPTIONS

  -f --force

    By default this will error out if the tokens have already been set.

    Use this flag to change the tokens.

## EXAMPLES

  `dishub init`

    On a fresh install or with a `-f` otherwise

      GitHub OAuth token: 994c365aec1700f5783bac697e2347ffd2268e1e
      Discord bot token: A8233f2465E4E27B36c3e9a9.5eC9Ab.e8FE135112B5f4d678BD1D221f8

      Remember to invite the bot to the servers you want it to post in!

  `dishub init`

    With tokens already set

      File "$HOME/.dishub/tokens.toml" was not overriden to prevent data loss.
      Pass --force to override it.

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/dishub/issues>&gt;

## SEE ALSO

&lt;<https://github.com/nabijaczleweli/dishub>&gt;
