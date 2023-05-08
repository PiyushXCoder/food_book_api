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

use super::{models, queries, utils};
use crate::helper::{
    self, db::DbState, db::DieselResultExt, jwt::Claims, responses::Message, result::ResultExt,
};

use actix_web::{get, http::StatusCode, post, put, web, HttpResponse, Result as ActixResult};
use actix_web_validator::Query;
use diesel::prelude::*;
use serde_json::json;

#[post("/user")]
pub(crate) async fn init_create_user(
    params: Query<queries::InitCreateUser>,
) -> ActixResult<HttpResponse> {
    let token = helper::jwt::AccountCreationClaims {
        first_name: params.first_name.clone(),
        last_name: params.last_name.clone(),
        email: params.email.clone(),
        avatar: params.avatar.clone(),
        password: params.password.clone(),
        jti: helper::uuid(),
        exp: helper::jwt::AccountCreationClaims::default_exp_duration(),
    }
    .encode()
    .map_err_log(|_| helper::errors::InternalServerError)?;

    let msg = format!(
        "Welcome to Food Book!\nYour account creation token:\n{}",
        token
    );

    utils::send_mail(&params.email, "Account Creation", &msg)?;

    return Ok(
        HttpResponse::Created().body(Message::new("Account creation email sent").to_string())
    );
}

#[put("/user")]
pub(crate) async fn create_user(
    params: Query<queries::CreateUser>,
    dbstate: web::Data<DbState>,
) -> ActixResult<HttpResponse> {
    use crate::schema::users;

    let claims = helper::jwt::AccountCreationClaims::decode(&params.token).map_err(|_| {
        helper::errors::CustomMessageServerError::new("Bad Token", StatusCode::BAD_REQUEST)
    })?;

    let mut con = dbstate.con()?;

    let records = models::CreateUser {
        first_name: claims.first_name.clone(),
        last_name: claims.last_name.clone(),
        email: claims.email.clone(),
        avatar: claims.avatar.clone(),
        password: helper::passwords::hash(&claims.password.clone())?,
        created_at: chrono::offset::Utc::now().naive_utc(),
    };

    diesel::insert_into(users::table)
        .values(records)
        .execute(&mut con)
        .validate_error()?;

    return Ok(HttpResponse::Created().body(Message::new("Account is created").to_string()));
}

#[get("/user/{user_id}")]
pub(crate) async fn get_user(
    id: web::Path<(i32,)>,
    dbstate: web::Data<DbState>,
) -> ActixResult<HttpResponse> {
    use crate::schema::users;

    let (id,) = id.into_inner();

    let mut con = dbstate.con()?;

    let record = users::table
        .select(models::GetUser::as_select())
        .filter(users::dsl::id.eq(id))
        .load(&mut con)
        .validate_error()?;

    let record = record
        .get(0)
        .ok_or_else(|| helper::errors::CustomMessageServerError {
            message: "User not found!".to_string(),
            status_code: StatusCode::NOT_FOUND,
        })?;
    return Ok(HttpResponse::Ok().body(
        serde_json::to_string(&record).map_err_log(|_| helper::errors::InternalServerError)?,
    ));
}

#[post("/login")]
pub(crate) async fn login(
    params: Query<queries::Login>,
    dbstate: web::Data<DbState>,
) -> ActixResult<HttpResponse> {
    use crate::schema::users;

    let mut con = dbstate.con()?;

    let record: Vec<models::LoginUser> = users::table
        .filter(users::dsl::email.eq(&params.email))
        .select(models::LoginUser::as_select())
        .limit(1)
        .load::<models::LoginUser>(&mut con)
        .validate_error()?;

    let record = match record.get(0) {
        Some(a) => a,
        None => {
            return Err(helper::errors::CustomMessageServerError::new(
                "Invalid Email",
                StatusCode::UNAUTHORIZED,
            )
            .into());
        }
    };

    if helper::passwords::verify(&params.password, &record.password)? {
        let token = helper::jwt::SessionClaims {
            user_id: record.id,
            first_name: record.first_name.clone(),
            last_name: record.last_name.clone().unwrap_or_default(),
            avatar: record.avatar.clone().unwrap_or_default(),
            exp: helper::jwt::SessionClaims::default_exp_duration(),
            jti: helper::uuid(),
        }
        .encode()
        .map_err_log(|_| helper::errors::InternalServerError)?;

        return Ok(HttpResponse::Created()
            .body(json!({ "token": token, "message": "Logged in" }).to_string()));
    } else {
        return Err(helper::errors::CustomMessageServerError::new(
            "Invalid Password",
            StatusCode::UNAUTHORIZED,
        )
        .into());
    }
}
