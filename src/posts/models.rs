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

use chrono::{NaiveDateTime, NaiveTime};
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::Serialize;

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub(crate) struct CreatePost {
    pub(crate) user_id: i32,
    pub(crate) heading: String,
    pub(crate) sub_heading: String,
    pub(crate) caption: String,
    pub(crate) cooking_duration: NaiveTime,
    pub(crate) tags: Vec<String>,
    pub(crate) visuals: Vec<String>,
    pub(crate) ingredients: Vec<String>,
    pub(crate) steps: Vec<String>,
    pub(crate) likes_count: i32,
    pub(crate) comments_count: i32,
    pub(crate) created_at: NaiveDateTime,
}

#[derive(Queryable, AsChangeset)]
#[diesel(table_name = posts)]
pub(crate) struct UpdatePost {
    pub(crate) heading: String,
    pub(crate) sub_heading: String,
    pub(crate) caption: String,
    pub(crate) cooking_duration: NaiveTime,
    pub(crate) tags: Vec<String>,
    pub(crate) visuals: Vec<String>,
    pub(crate) ingredients: Vec<String>,
    pub(crate) steps: Vec<String>,
}

#[derive(Queryable, Clone, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = posts)]

pub(crate) struct GetPost {
    pub(crate) id: i32,
    pub(crate) user_id: i32,
    pub(crate) heading: Option<String>,
    pub(crate) sub_heading: Option<String>,
    pub(crate) caption: Option<String>,
    pub(crate) cooking_duration: Option<NaiveTime>,
    pub(crate) tags: Option<Vec<Option<String>>>,
    pub(crate) visuals: Option<Vec<Option<String>>>,
    pub(crate) ingredients: Option<Vec<Option<String>>>,
    pub(crate) steps: Option<Vec<Option<String>>>,
    pub(crate) likes_count: Option<i32>,
    pub(crate) comments_count: Option<i32>,
    pub(crate) created_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Clone, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = posts)]

pub(crate) struct GetPosts {
    pub(crate) id: i32,
    pub(crate) user_id: i32,
    pub(crate) heading: Option<String>,
    pub(crate) sub_heading: Option<String>,
    pub(crate) cooking_duration: Option<NaiveTime>,
    pub(crate) tags: Option<Vec<Option<String>>>,
    pub(crate) visuals: Option<Vec<Option<String>>>,
    pub(crate) likes_count: Option<i32>,
    pub(crate) comments_count: Option<i32>,
    pub(crate) created_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = likes)]
pub(crate) struct AddLike {
    pub(crate) post_id: i32,
    pub(crate) user_id: i32,
    pub(crate) created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = comments)]
pub(crate) struct AddComment {
    pub(crate) post_id: i32,
    pub(crate) user_id: i32,
    pub(crate) note: String,
    pub(crate) created_at: NaiveDateTime,
}

#[derive(Queryable, Clone, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Post, foreign_key = post_id))]
#[diesel(table_name = comments)]
pub(crate) struct Comment {
    pub(crate) id: i32,
    pub(crate) post_id: i32,
    pub(crate) user_id: i32,
    pub(crate) note: Option<String>,
    pub(crate) created_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable, Debug, PartialEq, Serialize)]
#[diesel(table_name = users)]
pub(crate) struct User {
    pub(crate) id: i32,
    pub(crate) first_name: String,
    pub(crate) last_name: Option<String>,
    pub(crate) avatar: Option<String>,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize)]
#[diesel(table_name = posts)]
pub(crate) struct Post {
    pub(crate) id: i32,
    pub(crate) user_id: i32,
    pub(crate) heading: Option<String>,
    pub(crate) sub_heading: Option<String>,
    pub(crate) caption: Option<String>,
    pub(crate) cooking_duration: Option<NaiveTime>,
    pub(crate) tags: Option<Vec<Option<String>>>,
    pub(crate) visuals: Option<Vec<Option<String>>>,
    pub(crate) ingredients: Option<Vec<Option<String>>>,
    pub(crate) steps: Option<Vec<Option<String>>>,
    pub(crate) likes_count: Option<i32>,
    pub(crate) comments_count: Option<i32>,
    pub(crate) created_at: Option<NaiveDateTime>,
}
