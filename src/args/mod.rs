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

use clap::App;
use clap::Arg;
use clap::SubCommand;

/// Possible commands
pub enum Command {
    /// Got a command that we don't know
    Unknown,
    /// Fetch favourites from all accounts
    FetchAll,
    /// Fetch favourites from a specific account
    Fetch(String),
    /// Add a new account with the specified name
    CreateAccount(String),
    /// Remove the account with the specified name
    RemoveAccount(String),
}

/// Parse the command line, returning the necessary command.
pub fn parse() -> Command {
    let parser = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(Arg::with_name("account").help("Account alias"))
        .subcommand(
            SubCommand::with_name("create").about("Create the account"),
        )
        .subcommand(
            SubCommand::with_name("remove").about("Remove the account"),
        )
        .subcommand(
            SubCommand::with_name("fetch")
            .about("Fetch new favourites from this account only")
        )
        .subcommand(
            SubCommand::with_name("storage")
            .about("Account storage")
            .subcommand(SubCommand::with_name("add")
                .about("Add a new storage for the account")
                .arg(Arg::with_name("type")
                    .help("Storage type; valid types are: \"filesystem\"")
                    .takes_value(true)
                    .required(true)))
            .subcommand(SubCommand::with_name("remove")
                .about("Remove a storage from the account")
                .arg(Arg::with_name("type")
                    .help("Storage type to be removed")
                    .takes_value(true)
                    .required(true))));
    let matches = parser.get_matches();
    if let Some(account) = matches.value_of("account") {
        match matches.subcommand() {
            ("fetch", _) => Command::Fetch(account.into()),
            ("create", _) => Command::CreateAccount(account.into()),
            ("remove", _) => Command::RemoveAccount(account.into()),
            _ => Command::Unknown,
        }
    } else {
        log::debug!("No account provided, assuming fetch");
        Command::FetchAll
    }
}
