use std::io;

use crate::storage::data::Data;
use crate::storage::filesystem::Filesystem;
use elefren::helpers::cli;
use elefren::helpers::toml as elefren_toml;
use elefren::prelude::*;

mod config;
mod storage;

fn main() {
    let config = dbg!(config::Config::get());
    let client = dbg!(get_mastodon_connection());
    let top = dbg!(config.last_favorite.to_string());
    // let _joplin = crate::storage::joplin::validate(&config);
    let save_to = Filesystem::new();

    let most_recent_favourite = client
        .favourites()
        .unwrap()
        .items_iter()
        .take_while(|record| dbg!(record).id != top)
        .map(|record| {
            let conversion = dbg!(Data::from(dbg!(&record)));
            conversion.save(&save_to);
            record
        })
        .fold(None, {
            |first, current| {
                if first.is_some() {
                    first
                } else {
                    Some(current.id)
                }
            }
        });

    config.save(most_recent_favourite);
}

/// Get a connection with Mastodon; if there is no set up with any account yet,
/// requests one.
fn get_mastodon_connection() -> Mastodon {
    if let Ok(data) = elefren_toml::from_file("mastodon.toml") {
        Mastodon::from(data)
    } else {
        println!("Your server URL: ");
        let mut server = String::new();
        io::stdin()
            .read_line(&mut server)
            .expect("You need to enter yoru server URL");

        let registration = Registration::new(server.trim())
            .client_name("downfav")
            .build()
            .unwrap();
        let mastodon = cli::authenticate(registration).unwrap();
        elefren_toml::to_file(&*mastodon, "mastodon.toml").unwrap();
        mastodon
    }
}
