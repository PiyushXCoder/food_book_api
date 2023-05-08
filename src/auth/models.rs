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

use crate::schema::*;

use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub(crate) struct CreateUser {
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) email: String,
    pub(crate) avatar: Option<String>,
    pub(crate) password: String,
    pub(crate) created_at: NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
pub(crate) struct LoginUser {
    pub(crate) id: i32,
    pub(crate) first_name: String,
    pub(crate) last_name: Option<String>,
    pub(crate) avatar: Option<String>,
    pub(crate) password: String,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = users)]
pub(crate) struct GetUser {
    pub(crate) id: i32,
    pub(crate) first_name: String,
    pub(crate) last_name: Option<String>,
    pub(crate) email: String,
    pub(crate) avatar: Option<String>,
}
