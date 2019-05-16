use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

use elefren::prelude::*;
use elefren::helpers::cli;
use elefren::helpers::toml;
use elefren::entities::status;

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
        .for_each(move |record| dump_record(record))
        ;

    // status
    // status.account.acct (username)
    // status.id (id)
    // status.content
    // status.media_attachments
    //  -> attachment.remote_url / attachment.url
    //     attachment.
}

fn dump_record(record: status::Status) -> () {
    create_structure(&record);
    save_content(&record);
}

fn toot_dir(record: &status::Status) -> PathBuf {
    Path::new("data")
        .join(&record.account.acct)
        .join(&record.id)
}

fn create_structure(record: &status::Status) -> () {
    std::fs::create_dir_all(toot_dir(record))
        .expect("Failed to create the storage path");
}

fn save_content(record: &status::Status) -> () {
    if let Ok(mut fp) = File::create(toot_dir(&record).join("toot.md")) {
        fp.write_all(html2md::parse_html(&record.content).as_bytes())
            .expect("Failed to save content");
    }
}
