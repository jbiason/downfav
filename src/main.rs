use elefren::prelude::*;
use elefren::helpers::cli;
use elefren::helpers::toml;

fn main() {
    let client = if let Ok(data) = toml::from_file("mastodon.toml") {
        Mastodon::from(data)
    } else {
        let registration = Registration::new("https://functional.cafe")
            .client_name("downfav")
            .build()
            .unwrap();
        let mastodon = cli::authenticate(registration).unwrap();
        toml::to_file(&*mastodon, "mastodon.toml").unwrap();
        mastodon
    };

    client
        .favourites().unwrap()
        .items_iter()
        .take(2)
        .for_each(move |record| println!("{:#?}", record))
        ;

    // status
    // status.account.acct (username)
    // status.id (id)
    // status.content
    // status.media_attachments
    //  -> attachment.remote_url / attachment.url
    //     attachment.
}
