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

use super::commands::addaccount::AddAccount;
use super::commands::Command;

/// Parse the command line, returning the necessary command.
pub fn parse() -> Box<dyn Command> {
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
            SubCommand::with_name("sync")
                .about("Sync the last seen favourite with the most recent one")
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
    Box::new(AddAccount::new("something"))
}
