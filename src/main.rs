/*
   DOWNFAV - Download Favourites
   Copyright (C) 2020  Julio Biason

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU Affero General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Affero General Public License for more details.

   You should have received a copy of the GNU Affero General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::io;

use elefren::helpers::cli;
use elefren::helpers::toml as elefren_toml;
use elefren::prelude::*;

use crate::storage::data::Data;
use crate::storage::filesystem::Filesystem;
use crate::storage::joplin::Joplin;

mod config;
mod storage;

fn main() {
    let config = dbg!(config::Config::get());
    let client = dbg!(get_mastodon_connection());
    let top = dbg!(config.last_favorite.to_string());
    let joplin_storage = if let Some(joplin_config) = &config.joplin {
        Some(Joplin::new_from_config(&joplin_config))
    } else {
        None
    };
    let fs_storage = Filesystem::new();

    let most_recent_favourite = client
        .favourites()
        .unwrap()
        .items_iter()
        .take_while(|record| dbg!(record).id != top)
        .map(|record| {
            let conversion = dbg!(Data::from(dbg!(&record)));
            if let Some(joplin) = &joplin_storage {
                conversion.save(joplin);
            } else {
                conversion.save(&fs_storage);
            }
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
