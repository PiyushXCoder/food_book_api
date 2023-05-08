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

use super::errors;
use super::result::ResultExt;

use actix_web::{error::Error as ActixError, http::StatusCode, Result as ActixResult};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use log::error;
use std::panic::Location;

pub(crate) type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub(crate) struct DbState {
    pub(crate) dbpool: DbPool,
}

impl DbState {
    pub(crate) fn new() -> Self {
        let mgr = ConnectionManager::<PgConnection>::new(&crate::CONFIG.database.url);
        let dbpool = Pool::builder()
            .test_on_check_out(true)
            .build(mgr)
            .expect("Failed to connect with database");
        Self { dbpool }
    }

    pub(crate) fn con(&self) -> ActixResult<PooledConnection<ConnectionManager<PgConnection>>> {
        self.dbpool
            .get()
            .map_err_log(|_| errors::InternalServerError.into())
    }
}

pub(crate) fn generate_error_message(
    e: diesel::result::Error,
    loc: &Location<'static>,
) -> ActixError {
    match e {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::ForeignKeyViolation,
            e,
        ) => {
            return errors::CustomMessageServerError::new(
                &format!("{} is wrong", e.column_name().unwrap_or_else(|| "request")),
                StatusCode::BAD_REQUEST,
            )
            .into();
        }
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            e,
        ) => {
            return errors::CustomMessageServerError::new(
                &format!(
                    "{} is already used",
                    e.column_name().unwrap_or_else(|| "request")
                ),
                StatusCode::BAD_REQUEST,
            )
            .into();
        }
        _ => {
            error!("{}\n{:?}", loc, e);
            return errors::InternalServerError.into();
        }
    }
}

pub(crate) trait DieselResultExt<T> {
    fn validate_error(self) -> ActixResult<T>;
}

impl<T> DieselResultExt<T> for diesel::result::QueryResult<T> {
    #[track_caller]
    fn validate_error(self) -> ActixResult<T> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                error!("{}\n{:?}", Location::caller(), e);
                Err(generate_error_message(e, Location::caller()))
            }
        }
    }
}
