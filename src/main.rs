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

mod args;
mod commands;
mod config;
mod filesystem;
mod storage;

fn main() {
    env_logger::init();

    match args::parse() {
        Ok(command) => command.execute().unwrap(),
        Err(e) => println!("Error: {:?}", e),
    }
}

// Retrieve favourites
// fn fetch_favourites() {
//     let config = match config::AccountConfig::get() {
//         Ok(config) => config,
//         Err(e) => {
//             log::debug!("Configuration error: {:?}", e);
//             let data = connect_to_mastodon();
//             config::AccountConfig::from(data)
//         }
//     };

//     let top = config.favourite.last.to_string();
//     log::debug!("Last favourite seen: {}", top);
//     let storage: Box<dyn Storage> = match (&config.joplin, &config.org) {
//         (Some(joplin), _) => Box::new(Joplin::new_from_config(&joplin)),
//         (None, Some(org)) => Box::new(Org::new_from_config(&org)),
//         (None, None) => Box::new(Filesystem::new()),
//     };

//     let client = Mastodon::from(config.mastodon.clone());
//     let most_recent_favourite = client
//         .favourites()
//         .unwrap()
//         .items_iter()
//         .take_while(|record| {
//             println!("Current ID: {} (last favourite: {})", record.id, top);
//             record.id != top
//         })
//         .map(|record| {
//             log::debug!("Incoming record: {:?}", record);
//             let conversion = Data::from(&record);
//             log::debug!("Converted record: {:?}", conversion);
//             storage.save(&conversion);
//             record
//         })
//         .fold(None, {
//             |first, current| -> Option<String> {
//                 if first.is_some() {
//                     first
//                 } else {
//                     Some(current.id)
//                 }
//             }
//         });

//     if let Some(new_favourite) = most_recent_favourite {
//         config.save(&new_favourite);
//     }
// }
