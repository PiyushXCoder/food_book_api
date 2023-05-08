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

use chrono::NaiveTime;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub(crate) struct CreateUpdatePost {
    pub(crate) heading: String,
    pub(crate) sub_heading: String,
    pub(crate) caption: String,
    pub(crate) cooking_duration: NaiveTime,
    pub(crate) tags: Vec<String>,
    pub(crate) visuals: Vec<String>,
    pub(crate) ingredients: Vec<String>,
    pub(crate) steps: Vec<String>,
}

#[derive(Deserialize, Validate)]
pub(crate) struct AddComment {
    pub(crate) note: String,
}

#[derive(Deserialize, Validate)]
pub(crate) struct PostsQuery {
    pub(crate) user_id: Option<i32>,
    pub(crate) name: Option<String>,
}
