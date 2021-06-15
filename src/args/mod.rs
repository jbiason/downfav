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
use clap::ArgMatches;
use clap::SubCommand;

/// Possible commands
pub enum Command {
    /// Got a command that we don't know
    Unknown,
    /// Fetch all new favourites
    Fetch,
    /// Add a new account with the specified name
    AddAccount(String),
    /// Remove the account with the specified name
    RemoveAccount(String),
}

/// Parse the command line, returning the necessary command.
pub fn parse() -> Command {
    let parser = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .subcommand(
            SubCommand::with_name("fetch").about("Fetch the new favourites"),
        )
        .subcommand(
            SubCommand::with_name("account")
                .about("Manage Mastodon accounts")
                .subcommand(
                    SubCommand::with_name("add")
                        .about("Add a new account")
                        .arg(
                            Arg::with_name("name")
                                .required(true)
                                .takes_value(true)
                                .help("Account name"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("remove")
                        .about("Remove an account")
                        .arg(
                            Arg::with_name("name")
                                .required(true)
                                .takes_value(true)
                                .help("Name of the account to be removed"),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("storage")
                .about("Storage management")
                .arg(
                    Arg::with_name("account")
                        .required(true)
                        .takes_value(true)
                        .help("Account name"),
                )
                .subcommand(
                    SubCommand::with_name("add")
                        .about("Add a storage for an account")
                        .subcommand(SubCommand::with_name("filesystem")
                            .about("Stores toots in the filesystem")
                            .arg(Arg::with_name("path")
                                .required(true)
                                .takes_value(true)
                                .help("Path where store toots in the filesystem")))));
    let matches = parser.get_matches();
    match matches.subcommand() {
        ("", _) => Command::Fetch,
        ("fetch", _) => Command::Fetch,
        ("account", Some(arguments)) => parse_account(arguments),
        _ => Command::Unknown,
    }
}

fn parse_account(arguments: &ArgMatches) -> Command {
    log::debug!("Parsing accounts");
    match arguments.subcommand() {
        ("add", Some(argument)) => {
            log::debug!("Must add new account");
            let name = argument.value_of("name").unwrap();
            Command::AddAccount(name.into())
        }
        ("remove", Some(argument)) => {
            log::debug!("Must remove account");
            let name = argument.value_of("name").unwrap();
            Command::RemoveAccount(name.into())
        }
        _ => Command::Unknown,
    }
}
