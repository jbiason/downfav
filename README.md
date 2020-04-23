# DOWNload FAVourites

`downfav` is a simple script to download your Mastodon favourites, either to
the disk or into [Joplin](http://joplinapp.org/).

## Running

Simply call `downfav`. On the first run, it will ask for your server and do
the rounds into approving the app. After that, it will download every toot
marked as Favourite to the disk.

## Configuration

There are two configuration files:

* `mastodon.toml` contains the secret to access your server. 
* `downfav.toml` have the information used by downfav itself.

## `downfav.toml`

This file has the following format:

```
last_favorite = "<id>"

[joplin]
port = <port>
token = "<token>"
folder = "<folder>"
```

The first value, `last_favorite` is used by Downfav itself; as soon as it
completes a run, it will update this value (and create this configuration
file, if it doesn't exist) with the top favourite download. This is done to
prevent download favourites over and over again.

The next section, `[joplin]` must be added manually, if you want to save your
favorites into Joplin. The first two fields are the same used by the web
clipper, and you can check those two directly in the configuration panel. The
last one is the name of the notebook in which you want to keep the toots.
