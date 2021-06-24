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

mod errors;

use clap::App;
use clap::Arg;
use clap::SubCommand;

use self::errors::ParsingError;
use super::commands::Command;

/// Parse the command line, returning the necessary command.
pub fn parse() -> Result<Command, ParsingError> {
    let parser = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("account")
                .help("Account alias")
                .required(true),
        )
        .subcommand(SubCommand::with_name("create").about("Create the account"))
        .subcommand(SubCommand::with_name("remove").about("Remove the account"))
        .subcommand(
            SubCommand::with_name("fetch")
                .about("Fetch new favourites from this account only"),
        )
        .subcommand(
            SubCommand::with_name("sync")
                .about("Sync the last seen favourite with the most recent one"),
        )
        .subcommand(
            SubCommand::with_name("storage")
                .about("Account storage")
                .subcommand(
                    SubCommand::with_name("add")
                        .about("Add a new storage for the account")
                        .subcommand(SubCommand::with_name("markdown").about(
                            "Store favourites on the filesystem, as markdown",
                        ))
                        .subcommand(SubCommand::with_name("org").about(
                            "Store favourites on the filesystem, as Org files",
                        ))
                        .subcommand(
                            SubCommand::with_name("joplin")
                                .about("Store favourites on Joplin"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("remove")
                        .about("Remove a storage from the account")
                        .subcommand(
                            SubCommand::with_name("markdown")
                                .about("Remove the markdown storage"),
                        )
                        .subcommand(
                            SubCommand::with_name("org")
                                .about("Remove the org storage"),
                        )
                        .subcommand(
                            SubCommand::with_name("joplin")
                                .about("Remove the joplin storage"),
                        ),
                ),
        );

    let matches = parser.get_matches();
    let account_name = matches.value_of("account").unwrap(); // we can unwrap 'cause it is required
    match matches.subcommand() {
        ("create", _) => Ok(Command::add_account(account_name.into())),
        ("remove", _) => Ok(Command::remove_account(account_name.into())),
        _ => Err(ParsingError::UnknownCommand),
    }
}
