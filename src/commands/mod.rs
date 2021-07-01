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

use std::convert::TryFrom;
use std::io;
use std::io::prelude::*;

use elefren::helpers::cli;
use elefren::prelude::*;

use self::errors::CommandError;
use crate::config::config::Config;
use crate::config::Configurable;
use crate::storage::markdown::config::MarkdownConfig;

type CommandResult = Result<(), CommandError>;

/// Available Storages
pub enum StorageType {
    /// Store in the filesystem, as Markdown
    Markdown,

    /// Store in the filesystem, as Org-Mode
    Org,

    /// Store in Joplin
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

/// Available commands
pub enum Command {
    /// Add a new account
    AddAccount(String),

    /// Remove an account
    RemoveAccount(String),

    /// Add a storage in an account
    AddStorage(String, StorageType),

    /// Fetch favourites from all accounts
    FetchAll,
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

    /// Execute the command, based on its value
    pub fn execute(&self) -> CommandResult {
        match self {
            Command::AddAccount(name) => add_account(name),
            Command::RemoveAccount(name) => remove_account(name),
            Command::AddStorage(account, storage) => {
                add_storage(account, storage)
            }
            Command::FetchAll => fetch_all(),
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
    let mut config = Config::open()?;
    for (name, account_config) in config {
        log::debug!("Fetching new items from {:?}", name);
        if let Some(markdown_config) = account_config.markdown {
            log::debug!(
                "Markdown set to download to {:?}",
                markdown_config.path
            );
        }
    }
    Ok(())
}
