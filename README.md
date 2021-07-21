# DOWNload FAVourites

`downfav` is a simple application to download your Mastodon favourites, either
in Org or Markdown formats.

## Running

At first, running `downfav` should display nothing. The reason is that there
are no accounts or storage options for those accounts.

To create an account you need to run `downfav <accountalias> create`. This will
start the registration process for that account.

Next, you need to define where you want your favourites to be saved. To do
this, use `downfav <accountalias> storage add <storagetype>`. Currently, there
are two storage types: `markdown` and `org`.

### The Markdown Storage

The Markdown storage uses a directory structure based on the account name and
toot id. This means that, if you favourited a toot by "someuser@server"
identified by "123123", a tree like `<base storage
directory>/someuser@server/123123` will be created and the content will be
saved there.

(This storage is usually recommended if you normally favourite content with
lots of attachments, as each toot attachment -- image/video -- will be stored
alongside the toot text.)

### The Org Storage

The Org storage is similar to the Markdown storage, but instead of creating a
new file for each toot, every favourite will be added to a `<base storage
directory>/<date>.org` file; any attachments will be stored (and properly
linked) in `<base storage directory>/date/` directory.

(This storage is usually recommended if you normally favourite content with
lots of text and not much of attachments.)

## License

GNU AFFERO GENERAL PUBLIC LICENSE, Version 3.
