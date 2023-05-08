/*
 * This file is part of Food Book API.
 *
 * Food Book API is free software: you can redistribute it and/or modify it under the terms
 * of the GNU General Public License as published by the Free Software Foundation,
 * either version 3 of the License, or (at your option) any later version.
 *
 * Food Book API is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
 * PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with Food Book API.
 * If not, see <https://www.gnu.org/licenses/>.
 */

use std::path::PathBuf;

use clap::Parser;
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CliParams {
    /// Path of the configuration file
    #[arg(short, long)]
    config_file: PathBuf,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ConfigContaner {
    pub(crate) database: Database,
    pub(crate) server: Server,
    pub(crate) salts: Salts,
    pub(crate) smtp: Smtp,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Server {
    pub(crate) address: String,
    pub(crate) shared_dir: PathBuf,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Database {
    pub(crate) url: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Salts {
    pub(crate) jwt: String,
    pub(crate) password: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Smtp {
    pub(crate) from_address: String,
    pub(crate) server: String,
    pub(crate) username: String,
    pub(crate) password: String,
}

lazy_static! {
    pub(crate) static ref CONFIG: ConfigContaner =
        toml::from_str(&std::fs::read_to_string(&CliParams::parse().config_file).unwrap()).unwrap();
}
