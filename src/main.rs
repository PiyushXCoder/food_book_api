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

pub(crate) mod auth;
mod config;
pub(crate) mod fs;
pub(crate) mod helper;
pub(crate) mod posts;
pub(crate) mod schema;

pub(crate) use config::CONFIG;
use helper::jwt::Claims;

use actix_web::http::StatusCode;
use actix_web::{dev::ServiceRequest, middleware::Logger, web, App, Error, HttpServer};
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    lazy_static::initialize(&CONFIG);

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);

        App::new()
            .app_data(web::Data::new(helper::db::DbState::new()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}"))
            .service(
                web::scope("/api/s")
                    .wrap(auth)
                    .configure(auth::urls::config_with_auth)
                    .configure(posts::urls::config_with_auth)
                    .configure(fs::urls::config_with_auth),
            )
            .service(
                web::scope("/api/o")
                    .configure(posts::urls::config_without_auth)
                    .configure(auth::urls::config_without_auth)
                    .configure(fs::urls::config_without_auth),
            )
    })
    .bind(&CONFIG.server.address)?
    .run()
    .await
}

async fn validator(
    req: ServiceRequest,
    credientials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let result = helper::jwt::SessionClaims::decode(credientials.token());
    match result {
        Ok(_) => {
            req.attach(vec!["LOGGEDIN".to_string()]);
            return Ok(req);
        }
        Err(_) => Err((
            helper::errors::CustomMessageServerError::new(
                "Not Permitted",
                StatusCode::UNAUTHORIZED,
            )
            .into(),
            req,
        )),
    }
}
