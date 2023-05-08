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

use super::super::{models, queries};
use crate::helper::db::DieselResultExt;
use crate::helper::jwt::Claims;
use crate::helper::{self, db::DbState, responses::Message, result::ResultExt};

use actix_web::{delete, get, post, web, HttpResponse, Result as ActixResult};
use actix_web_grants::proc_macro::has_any_permission;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_validator::Query;
use diesel::prelude::*;
use serde_json::json;

#[post("/post/{post_id}/comment")]
#[has_any_permission("LOGGEDIN")]
pub(crate) async fn add_comment(
    id: web::Path<(i32,)>,
    dbstate: web::Data<DbState>,
    params: Query<queries::AddComment>,
    ath: BearerAuth,
) -> ActixResult<HttpResponse> {
    use crate::schema::comments;
    let (id,) = id.into_inner();

    let mut con = dbstate.con()?;

    let claims: helper::jwt::SessionClaims =
        helper::jwt::SessionClaims::decode(ath.token()).unwrap();

    let records = models::AddComment {
        post_id: id,
        user_id: claims.user_id,
        note: params.note.clone(),
        created_at: chrono::offset::Utc::now().naive_utc(),
    };

    let id: i32 = diesel::insert_into(comments::table)
        .values(records)
        .returning(comments::dsl::id)
        .get_result::<i32>(&mut con)
        .validate_error()?;

    return Ok(HttpResponse::Created()
        .body(json!({ "id": id, "message": "Comment is added" }).to_string()));
}

#[delete("/post/comment/{comment_id}")]
#[has_any_permission("LOGGEDIN")]
pub(crate) async fn delete_comment(
    id: web::Path<(i32,)>,
    dbstate: web::Data<DbState>,
    ath: BearerAuth,
) -> ActixResult<HttpResponse> {
    use crate::schema::comments;
    let (id,) = id.into_inner();

    let mut con = dbstate.con()?;

    let claims: helper::jwt::SessionClaims =
        helper::jwt::SessionClaims::decode(ath.token()).unwrap();

    diesel::delete(comments::table)
        .filter(comments::dsl::id.eq(id))
        .filter(comments::dsl::user_id.eq(claims.user_id))
        .execute(&mut con)
        .validate_error()?;

    return Ok(HttpResponse::Created().body(Message::new("Comment is deleted").to_string()));
}

#[get("/post/{post_id}/comments")]
#[has_any_permission("LOGGEDIN")]
pub(crate) async fn get_comments(
    params: Query<helper::queries::Paginator>,
    dbstate: web::Data<DbState>,
) -> ActixResult<HttpResponse> {
    use crate::schema::{comments, posts, users};

    let mut con = dbstate.con()?;

    let record: Vec<(models::Comment, models::User)> = comments::table
        .inner_join(users::table)
        .inner_join(posts::table)
        .limit(params.limit)
        .offset(params.offset)
        .select((models::Comment::as_select(), models::User::as_select()))
        .load(&mut con)
        .validate_error()?;

    let record =
        serde_json::to_string(&record).map_err_log(|_| helper::errors::InternalServerError)?;
    return Ok(HttpResponse::Created().body(record));
}
