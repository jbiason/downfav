# DOWNload FAVourites

`downfav` is a simple script to download your Mastodon favourites, either to
the disk or into [Joplin](http://joplinapp.org/).

## Running

Simply call `downfav`. On the first run, it will ask for your server and do
the rounds into approving the app. After that, it will download every toot
marked as Favourite to the disk.

## Configuration

There is one single configuration file, `downfav.toml` which is read in the
current directory.

The general format of this file is:

```toml
[favourite]
last = "<id>"

[mastodon]
base = "<server>"
client_id = "<id>"
client_secret = "<secret>"
redirect = "<oauth id>"
token = "<access token>"

[org]
location = "<some path>"

[joplin]
port = <port>
token = "<token>"
folder = "<folder>"
```

When you run `downfav`, it will ask for your server an ask to connect to your
account. After that, the first two sections will be added.

By default, `downfav` stores favourites as Markdown files in a directory called
`data` along the configuration file; this is called the Filesystem storage.
Besides this storage, `downfav` have two more storages:

### Joplin Storage

[Joplin](https://joplinapp.org/) is an open source note taking application.
To use Joplin as a storage for `downfav`, you need to enable the web clipper.
On the same page, you'll find the port (usually is 41184) and the token. Along
with that, you need to define a Folder where all the favourites will be stored,
using its name.

Once you have this information, you can add the `[joplin]` section and the rest
of the information.

### Org Storage

[Org-Mode](https://orgmode.org/) is a format/plugin for Emacs that can keep
notes, agenda, TODOs and a bunch more.

To enable Org mode, you need to add the `[org]` section and define the path
where the notes will be kept. `downfav` will create one note per day, adding
any new favourites in that note.

### Resolution order

But what happens if I have Joplin and Org set up in my config file? Well, by
default, `downfav` will pick Joplin and ignore Org. If you want to save on Org
format, you must not have the Joplin section in your config.

And, to use the Filesystem storage, you should have no other configuration.

## License

GNU AFFERO GENERAL PUBLIC LICENSE, Version 3.
