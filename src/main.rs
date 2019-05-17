use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

use elefren::entities::attachment::Attachment;
use elefren::entities::status::Status;
use elefren::helpers::cli;
use elefren::helpers::toml;
use elefren::prelude::*;

use reqwest;

use hyper::Uri;

use log;
use env_logger;

fn main() {
    env_logger::init();

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

    log::info!("Starting up...");
    client
        .favourites()
        .unwrap()
        .items_iter()
        .take(2)
        .for_each(move |record| dump_record(record));
}

fn dump_record(record: Status) -> () {
    log::debug!("Retriving record {}", record.id);
    create_structure(&record);
    save_content(&record);
    save_attachments(&record);
}

fn toot_dir(record: &Status) -> PathBuf {
    Path::new("data")
        .join(&record.account.acct)
        .join(&record.id)
}

fn create_structure(record: &Status) -> () {
    std::fs::create_dir_all(toot_dir(record)).expect("Failed to create the storage path");
}

fn save_content(record: &Status) -> () {
    if let Ok(mut fp) = File::create(toot_dir(&record).join("toot.md")) {
        log::debug!("Saving content of {}..", record.id);
        fp.write_all(html2md::parse_html(&record.content).as_bytes())
            .expect("Failed to save content");
    }
}

fn save_attachments(record: &Status) -> () {
    log::debug!("Saving attachments of {}...", record.id);
    let base_path = toot_dir(&record);
    record
        .media_attachments
        .iter()
        .for_each(move |x| save_attachment(&x, &base_path));
}

fn save_attachment(attachment: &Attachment, base_path: &PathBuf) -> () {
    log::debug!("Saving attachment {}", attachment.url);
    let uri: Uri = attachment.url.parse().expect("Invalid URL");
    let body = reqwest::get(&attachment.url)
        .expect("Failed to connect to server")
        .text()
        .expect("Failed to retrieve attachment");

    if let Ok(mut fp) = File::create(base_path.join(uri.path())) {
        fp.write_all(body.as_bytes())
            .expect("Failed to save the attachment");
    }
}
