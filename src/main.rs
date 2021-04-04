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
use elefren::prelude::*;

use crate::storage::data::Data;
use crate::storage::filesystem::Filesystem;
use crate::storage::joplin::Joplin;
use crate::storage::storage::Storage;

mod config;
mod storage;

fn main() {
    let config = match config::Config::get() {
        Ok(config) => config,
        Err(_) => {
            let data = connect_to_mastodon();
            config::Config::from(data)
        }
    };

    let top = dbg!(config.favourite.last.to_string());
    let storage: Box<dyn Storage> = match &config.joplin {
        Some(joplin) => Box::new(Joplin::new_from_config(&joplin)),
        None => Box::new(Filesystem::new()),
    };

    let client = Mastodon::from(config.mastodon.clone());
    let most_recent_favourite = client
        .favourites()
        .unwrap()
        .items_iter()
        .take_while(|record| dbg!(record).id != top)
        .map(|record| {
            let conversion = dbg!(Data::from(dbg!(&record)));
            storage.save(&conversion);
            record
        })
        .fold(None, {
            |first, current| -> Option<String> {
                if first.is_some() {
                    first
                } else {
                    Some(current.id)
                }
            }
        });

    if let Some(new_favourite) = most_recent_favourite {
        config.save(&new_favourite);
    }
}

/// Create a connection to a mastodon server.
fn connect_to_mastodon() -> elefren::data::Data {
    println!("Your server URL: ");
    let mut server = String::new();
    io::stdin()
        .read_line(&mut server)
        .expect("You need to enter yoru server URL");

    let registration = Registration::new(server.trim())
        .client_name("downfav")
        .build()
        .unwrap();
    cli::authenticate(registration).unwrap().data
}
