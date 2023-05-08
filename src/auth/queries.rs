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

use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub(crate) struct InitCreateUser {
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    #[validate(email)]
    pub(crate) email: String,
    pub(crate) avatar: Option<String>,
    pub(crate) password: String,
}

#[derive(Deserialize, Validate)]
pub(crate) struct CreateUser {
    pub(crate) token: String,
}

#[derive(Deserialize, Validate)]
pub(crate) struct Login {
    pub(crate) email: String,
    pub(crate) password: String,
}
