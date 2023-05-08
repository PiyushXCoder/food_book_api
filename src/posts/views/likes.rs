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

use super::super::models;
use crate::helper::db::DieselResultExt;
use crate::helper::jwt::Claims;
use crate::helper::{self, db::DbState, responses::Message};

use actix_web::http::StatusCode;
use actix_web::{delete, post, web, HttpResponse, Result as ActixResult};
use actix_web_grants::proc_macro::has_any_permission;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use diesel::prelude::*;

#[post("/post/{post_id}/like")]
#[has_any_permission("LOGGEDIN")]
pub(crate) async fn add_like(
    id: web::Path<(i32,)>,
    dbstate: web::Data<DbState>,
    ath: BearerAuth,
) -> ActixResult<HttpResponse> {
    use crate::schema::likes;
    let (id,) = id.into_inner();

    let mut con = dbstate.con()?;

    let claims: helper::jwt::SessionClaims =
        helper::jwt::SessionClaims::decode(ath.token()).unwrap();

    let records = models::AddLike {
        post_id: id,
        user_id: claims.user_id,
        created_at: chrono::offset::Utc::now().naive_utc(),
    };

    diesel::insert_into(likes::table)
        .values(records)
        .execute(&mut con)
        .validate_error()?;

    return Ok(HttpResponse::Created().body(Message::new("Post is liked").to_string()));
}

#[delete("/post/{post_id}/like")]
#[has_any_permission("LOGGEDIN")]
pub(crate) async fn delete_like(
    id: web::Path<(i32,)>,
    dbstate: web::Data<DbState>,
    ath: BearerAuth,
) -> ActixResult<HttpResponse> {
    use crate::schema::likes;
    let (id,) = id.into_inner();

    let mut con = dbstate.con()?;

    let claims: helper::jwt::SessionClaims =
        helper::jwt::SessionClaims::decode(ath.token()).unwrap();

    let changes = diesel::delete(
        likes::table
            .filter(likes::dsl::post_id.eq(id))
            .filter(likes::dsl::user_id.eq(claims.user_id)),
    )
    .execute(&mut con)
    .validate_error()?;

    if changes == 0 {
        return Err(helper::errors::CustomMessageServerError {
            message: "Post not found!".to_string(),
            status_code: StatusCode::NOT_FOUND,
        }
        .into());
    }

    return Ok(HttpResponse::Created().body(Message::new("Post is unliked").to_string()));
}
