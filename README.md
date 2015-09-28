# github-sweep

Simple app to clear GitHub notifications.

If, like me, you find the blue dot on the notification icon on GitHub to be distracting, then this app will automatically mark your notifications as read every five seconds.

## Building

Install [Rust](http://www.rust-lang.org), version 1.3 or greater.  Then:

```sh
cargo build
```

## Running

Create a [personal access token](https://help.github.com/articles/creating-an-access-token-for-command-line-use/), and set the `GITHUB_SWEEP_USER` and `GITHUB_SWEEP_TOKEN` environment variables.  Then:

```sh
cargo run
```
