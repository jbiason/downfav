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
    let filename = base_path.join(get_attachment_filename(&attachment.url));
    log::debug!("Saving attachment to {:?}", filename);
    if let Ok(mut fp) = File::create(filename) {
        reqwest::get(&attachment.url)
            .expect("Failed to connect to server")
            .copy_to(&mut fp)
            .expect("Failed to save attachment");
    }
}

fn get_attachment_filename(url: &str) -> String {
    let mut frags = url.rsplitn(2, '/');
    log::debug!("URL fragments: {:?}", frags);
    if let Some(path_part) = frags.next() { 
        log::debug!("Found path in the attachment URL: {:?}", path_part);
        path_part
            .split('?')
            .next()
            .unwrap_or(url)
            .to_string()
    } else {
        // this is, most of the time, bad (due special characters -- like '?' -- and path)
        log::debug!("No path in attachment, using full URL");
        url.to_string()
    }
}
