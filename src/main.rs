use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use elefren::entities::attachment::Attachment;
use elefren::entities::status::Status;
use elefren::helpers::cli;
use elefren::helpers::toml as elefren_toml;
use elefren::prelude::*;

use reqwest;

use toml;
use serde_derive::Serialize;
use serde_derive::Deserialize;

#[derive(Serialize, Deserialize)]
struct Config {
    last_favorite: String
}

fn main() {
    let client = if let Ok(data) = elefren_toml::from_file("mastodon.toml") {
        Mastodon::from(data)
    } else {
        println!("Your server URL: ");
        let mut server = String::new();
        io::stdin().read_line(&mut server)
            .expect("You need to enter yoru server URL");

        let registration = Registration::new(server.trim())
            .client_name("downfav")
            .build()
            .unwrap();
        let mastodon = cli::authenticate(registration).unwrap();
        elefren_toml::to_file(&*mastodon, "mastodon.toml").unwrap();
        mastodon
    };

    let top = get_top_favourite();

    let most_recent_favourite = client
        .favourites()
        .unwrap()
        .items_iter()
        .take_while(|record| record.id != top)
        .map(|record| { dump_record(&record); record })
        .fold(None, {|first, current| {
            if first.is_some() {
                first
            } else {
                Some(current.id)
            }
        }});

    if let Some(id) = most_recent_favourite {
        let new_configuration = Config { last_favorite: id };
        let content = toml::to_string(&new_configuration).unwrap();

        if let Ok(mut fp) = File::create("downfav.toml") {
            fp.write_all(content.as_bytes()).unwrap();
        }
    }
}

fn get_top_favourite() -> String {
    if let Ok(mut fp) = File::open("downfav.toml") {
        let mut contents = String::new();
        fp.read_to_string(&mut contents).unwrap();
        
        let config: Config = toml::from_str(&contents)
            .unwrap_or( Config { last_favorite: "".to_string() } );
        config.last_favorite
    } else {
        "".to_string()
    }
}

fn dump_record(record: &Status) {
    println!("Downloading {}/{}",
        &record.account.acct,
        &record.id);
    dbg!("Content:", record);
    create_structure(&record);
    save_content(&record);
    save_attachments(&record);
}

fn toot_dir(record: &Status) -> PathBuf {
    Path::new("data")
        .join(&record.account.acct)
        .join(&record.id)
}

fn create_structure(record: &Status) {
    std::fs::create_dir_all(toot_dir(record)).expect("Failed to create the storage path");
}

fn save_content(record: &Status) {
    if let Ok(mut fp) = File::create(toot_dir(&record).join("toot.md")) {
        dbg!("Saving content of", &record.id);
        fp.write_all(html2md::parse_html(&record.content).as_bytes())
            .expect("Failed to save content");
    }
}

fn save_attachments(record: &Status) {
    dbg!("Saving attachments of", &record.id);
    let base_path = toot_dir(&record);
    record
        .media_attachments
        .iter()
        .for_each(move |x| save_attachment(&x, &base_path));
}

fn save_attachment(attachment: &Attachment, base_path: &PathBuf) {
    dbg!("Saving attachment", &attachment.url);
    let filename = base_path.join(get_attachment_filename(&attachment.url));
    dbg!("Saving attachment to", &filename);
    println!("\tAttachment: {:?}", &filename);
    if let Ok(mut fp) = File::create(filename) {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(600))
            .build()
            .unwrap();
        client.get(&attachment.url)
            .send()
            .expect("Failed to connect to server")
            .copy_to(&mut fp)
            .expect("Failed to save attachment");
    }
}

fn get_attachment_filename(url: &str) -> String {
    let mut frags = url.rsplitn(2, '/');
    dbg!("URL fragments:", &frags);
    if let Some(path_part) = frags.next() {
        dbg!("Found path in the attachment URL:", &path_part);
        path_part
            .split('?')
            .next()
            .unwrap_or(url)
            .to_string()
    } else {
        // this is, most of the time, bad (due special characters -- like '?' -- and path)
        dbg!("No path in attachment, using full URL");
        url.to_string()
    }
}
