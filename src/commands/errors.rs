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

use crate::config::errors::ConfigError;

/// Errors for the commands
#[derive(Debug)]
pub enum CommandError {
    /// Error connecting to the Mastodon account
    ConnectError,

    /// Configuration file is broken
    ConfigError(ConfigError),

    /// The requested account does not exist
    NoSuchAccount,

    /// The storage type requested does not exist
    NoSuchStorage,

    /// No new favourite
    NoNewFavourite,
}

impl From<elefren::Error> for CommandError {
    fn from(_: elefren::Error) -> CommandError {
        CommandError::ConnectError
    }
}

impl From<ConfigError> for CommandError {
    fn from(e: ConfigError) -> CommandError {
        CommandError::ConfigError(e)
    }
}
