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

use super::Command;

use std::io;
use std::io::prelude::*;

use elefren::helpers::cli;
use elefren::prelude::*;

use crate::config::config::Config;

pub struct AddAccount {
    name: String,
}

impl AddAccount {
    pub fn new(name: &str) -> Self {
        Self { name: name.into() }
    }
}

impl Command for AddAccount {
    fn execute(&self) -> Result<(), ()> {
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
        config.add_account(&self.name, connection);
        config.save()?;
        Ok(())
    }
}
