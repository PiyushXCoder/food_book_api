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
use actix_web::http::StatusCode;
use actix_web::{delete, get, post, put, web, HttpResponse, Result as ActixResult};
use actix_web_grants::proc_macro::has_permissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_validator::{Json, Query};
use diesel::prelude::*;
use serde_json::json;

#[post("/post")]
#[has_permissions("LOGGEDIN")]
pub(crate) async fn create_post(
    params: Json<queries::CreateUpdatePost>,
    dbstate: web::Data<DbState>,
    ath: BearerAuth,
) -> ActixResult<HttpResponse> {
    use crate::schema::posts;

    let mut con = dbstate.con()?;

    let claims: helper::jwt::SessionClaims =
        helper::jwt::SessionClaims::decode(ath.token()).unwrap();

    let records = models::CreatePost {
        user_id: claims.user_id,
        heading: params.heading.clone(),
        sub_heading: params.sub_heading.clone(),
        caption: params.caption.clone(),
        cooking_duration: params.cooking_duration,
        tags: params.tags.clone(),
        visuals: params.visuals.clone(),
        ingredients: params.ingredients.clone(),
        steps: params.steps.clone(),
        likes_count: 0,
        comments_count: 0,
        created_at: chrono::offset::Utc::now().naive_utc(),
    };

    let id: i32 = diesel::insert_into(posts::table)
        .values(records)
        .returning(posts::dsl::id)
        .get_result::<i32>(&mut con)
        .validate_error()?;

    return Ok(
        HttpResponse::Created().body(json!({ "id": id, "message": "Post is created" }).to_string())
    );
}

#[put("/post/{post_id}")]
#[has_permissions("LOGGEDIN")]
pub(crate) async fn update_post(
    id: web::Path<(i32,)>,
    params: Json<queries::CreateUpdatePost>,
    dbstate: web::Data<DbState>,
) -> ActixResult<HttpResponse> {
    use crate::schema::posts;

    let mut con = dbstate.con()?;
    let (id,) = id.into_inner();

    let records = models::UpdatePost {
        heading: params.heading.clone(),
        sub_heading: params.sub_heading.clone(),
        caption: params.caption.clone(),
        cooking_duration: params.cooking_duration,
        tags: params.tags.clone(),
        visuals: params.visuals.clone(),
        ingredients: params.ingredients.clone(),
        steps: params.steps.clone(),
    };

    let changes = diesel::update(posts::table)
        .filter(posts::dsl::id.eq(id))
        .set(records)
        .execute(&mut con)
        .validate_error()?;

    if changes == 0 {
        return Err(helper::errors::CustomMessageServerError {
            message: "Post not found!".to_string(),
            status_code: StatusCode::NOT_FOUND,
        }
        .into());
    }

    return Ok(HttpResponse::Created().body(Message::new("Post is updated").to_string()));
}

#[delete("/post/{post_id}")]
#[has_permissions("LOGGEDIN")]
pub(crate) async fn delete_post(
    id: web::Path<(i32,)>,
    dbstate: web::Data<DbState>,
    ath: BearerAuth,
) -> ActixResult<HttpResponse> {
    use crate::schema::posts;

    let mut con = dbstate.con()?;
    let (id,) = id.into_inner();

    let claims: helper::jwt::SessionClaims =
        helper::jwt::SessionClaims::decode(ath.token()).unwrap();

    diesel::delete(posts::table)
        .filter(posts::dsl::id.eq(id))
        .filter(posts::dsl::user_id.eq(claims.user_id))
        .execute(&mut con)
        .validate_error()?;

    return Ok(HttpResponse::Created().body(Message::new("Post is deleted").to_string()));
}

#[get("/post/{id}")]
pub(crate) async fn get_post(
    id: web::Path<(i32,)>,
    dbstate: web::Data<DbState>,
) -> ActixResult<HttpResponse> {
    use crate::schema::{posts, users};
    let (id,) = id.into_inner();

    let mut con = dbstate.con()?;

    let record: Vec<(models::GetPost, models::User)> = posts::table
        .inner_join(users::table)
        .filter(posts::dsl::id.eq(id))
        .limit(1)
        .select((models::GetPost::as_select(), models::User::as_select()))
        .load(&mut con)
        .validate_error()?;

    let record = record
        .get(0)
        .ok_or_else(|| helper::errors::CustomMessageServerError {
            message: "Post not found!".to_string(),
            status_code: StatusCode::NOT_FOUND,
        })?;

    let record =
        serde_json::to_string(&record).map_err_log(|_| helper::errors::InternalServerError)?;
    return Ok(HttpResponse::Ok().body(record));
}

#[get("/posts")]
#[has_permissions("LOGGEDIN")]
pub(crate) async fn list_posts(
    params: Query<helper::queries::Paginator>,
    query: Query<queries::PostsQuery>,
    dbstate: web::Data<DbState>,
) -> ActixResult<HttpResponse> {
    use crate::schema::{posts, users};

    let mut con = dbstate.con()?;

    let record = posts::table
        .inner_join(users::table)
        .limit(params.limit)
        .offset(params.offset)
        .select((models::GetPosts::as_select(), models::User::as_select()));

    let record: Vec<(models::GetPosts, models::User)> =
        if query.user_id.is_some() && query.name.is_some() {
            record
                .filter(users::dsl::id.eq(query.user_id.unwrap()))
                .filter(posts::dsl::heading.ilike(&format!("%{}%", query.name.clone().unwrap())))
                .load(&mut con)
        } else if query.user_id.is_none() && query.name.is_some() {
            record
                .filter(posts::dsl::heading.ilike(&format!("%{}%", query.name.clone().unwrap())))
                .load(&mut con)
        } else if query.user_id.is_some() && query.name.is_none() {
            record
                .filter(users::dsl::id.eq(query.user_id.unwrap()))
                .load(&mut con)
        } else {
            record.load(&mut con)
        }
        .validate_error()?;

    let record =
        serde_json::to_string(&record).map_err_log(|_| helper::errors::InternalServerError)?;
    return Ok(HttpResponse::Ok().body(record));
}
