# Downfav

Download your favorites/likes from Mastodon/Twitter.

## Architecture

This is mostly an experiment in creating microservices with Rust.

* `template` is a microservice that receives a template name and some data and
  renders the template back to disk. Uses:
	* [Actic-web](https://github.com/actix/actix-web) for the web interface.
	* [Clap](https://github.com/clap-rs/clap) for the command line options.
	* [Log-Derive](https://github.com/elichai/log-derive) for logging.
