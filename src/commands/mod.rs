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

pub mod errors;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::io;
use std::io::prelude::*;

use elefren::helpers::cli;
use elefren::prelude::*;

use self::errors::CommandError;
use crate::config::account::AccountConfig;
use crate::config::config::Config;
use crate::config::Configurable;
use crate::storage::data::Data;
use crate::storage::markdown::config::MarkdownConfig;
use crate::storage::markdown::storage::Markdown;
use crate::storage::storage::Storage;

type CommandResult = Result<(), CommandError>;

/// Available Storages.
#[derive(Debug)]
pub enum StorageType {
    /// Store in the filesystem, as Markdown.
    Markdown,

    /// Store in the filesystem, as Org-Mode.
    Org,

    /// Store in Joplin.
    Joplin,
}

impl TryFrom<&str> for StorageType {
    type Error = errors::CommandError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "markdown" => Ok(StorageType::Markdown),
            "org" => Ok(StorageType::Org),
            "joplin" => Ok(StorageType::Joplin),
            _ => Err(Self::Error::NoSuchStorage),
        }
    }
}

/// Available commands.
#[derive(Debug)]
pub enum Command {
    /// Add a new account.
    AddAccount(String),

    /// Remove an account.
    RemoveAccount(String),

    /// Add a storage in an account.
    AddStorage(String, StorageType),

    /// Fetch favourites from all accounts.
    FetchAll,

    /// Fetch one single account.
    Fetch(String),

    /// Forces the last favourite to be the current favourite.
    Sync(String),
}

impl Command {
    pub fn add_account(name: &str) -> Self {
        Command::AddAccount(name.into())
    }

    pub fn remove_account(name: &str) -> Self {
        Command::RemoveAccount(name.into())
    }

    pub fn add_storage(account: &str, storage: StorageType) -> Self {
        Command::AddStorage(account.into(), storage)
    }

    pub fn fetch_all() -> Self {
        Command::FetchAll
    }

    pub fn fetch(account: &str) -> Self {
        Command::Fetch(account.into())
    }

    pub fn sync(account: &str) -> Self {
        Command::Sync(account.into())
    }

    /// Execute the command, based on its value
    pub fn execute(&self) -> CommandResult {
        match self {
            Command::AddAccount(name) => add_account(name),
            Command::RemoveAccount(name) => remove_account(name),
            Command::AddStorage(account, storage) => {
                add_storage(account, storage)
            }
            Command::FetchAll => fetch_all(),
            Command::Fetch(account) => fetch_account(account),
            Command::Sync(account) => sync_account(account),
        }
    }
}

fn add_account(name: &str) -> CommandResult {
    let mut server = String::new();

    print!("Your server URL: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut server)
        .expect("you need to ender your server URL");
    let registration = Registration::new(server.trim())
        .client_name("Downfav")
        .build()?;
    let connection = cli::authenticate(registration)?.data;

    let mut config = Config::open()?;
    config.add_account(&name, connection);
    config.save()?;
    Ok(())
}

fn remove_account(name: &str) -> CommandResult {
    let mut config = Config::open()?;
    config.remove_account(&name);
    config.save()?;
    Ok(())
}

fn add_storage(account: &str, storage: &StorageType) -> CommandResult {
    let mut config = Config::open()?;
    match storage {
        StorageType::Markdown => {
            let storage_config = MarkdownConfig::config()?;
            config.set_storage_markdown(account, storage_config);
        }
        _ => unimplemented!(),
    }
    config.save()?;
    Ok(())
}

fn fetch_all() -> CommandResult {
    // So, retrieve the favourites and get the latest seen...
    let config = Config::open()?;
    let mut favourites: HashMap<String, String> = HashMap::new();
    for (name, account_config) in config.into_iter() {
        log::debug!("Fetching new items from {:?}", name);
        match fetch_account_favourites(&account_config) {
            Some(new_favourite) => {
                favourites.insert(name.into(), new_favourite.into());
            }
            None => {}
        }
    }

    // ... and then update it in the configuration
    let mut config = Config::open()?;
    for (account, favourite) in favourites {
        config.set_new_favourite(&account, &favourite);
    }
    config.save()?;
    Ok(())
}

fn fetch_account(_account: &str) -> CommandResult {
    Ok(())
}

fn fetch_account_favourites(account: &AccountConfig) -> Option<String> {
    // XXX before anything, we could check if there is any storage enabled.
    // XXX we could create a list of storages, so after retrieving the toot
    //     and converting to our format, we just go through this list and call
    //     `.save()` in each.
    let top = account.top_favourite();
    let mut most_recent: Option<String> = None;
    let client = Mastodon::from(account.mastodon());
    let markdown_storage = match account.markdown() {
        Some(config) => Some(Markdown::new(&config)),
        None => None,
    };
    for toot in client.favourites().ok()?.items_iter() {
        if toot.id == top {
            break;
        }

        if most_recent.is_none() {
            most_recent = Some((&toot.id).into());
        }

        let conversion = Data::from(&toot);
        println!("Found new favourite: {}", conversion.id);

        if let Some(storage) = markdown_storage.as_ref() {
            storage.save(&conversion);
        }
    }
    most_recent
}

fn sync_account(_account: &str) -> CommandResult {
    Ok(())
}
