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

use crate::helper::responses;

use actix_web::{http::StatusCode, ResponseError};
use derive_more::Error;
use std::fmt::Display;

#[derive(Error, Debug)]
pub(crate) struct InternalServerError;

impl Display for InternalServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            responses::ErrorMessage::new_internal_server().to_string()
        )
    }
}

impl ResponseError for InternalServerError {}

#[derive(Error, Debug)]
pub(crate) struct CustomMessageServerError {
    pub(crate) message: String,
    pub(crate) status_code: StatusCode,
}

impl CustomMessageServerError {
    pub(crate) fn new(message: &str, status_code: StatusCode) -> Self {
        Self {
            message: message.to_owned(),
            status_code,
        }
    }
}

impl Display for CustomMessageServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            responses::ErrorMessage::new(&self.message).to_string()
        )
    }
}

impl ResponseError for CustomMessageServerError {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }
}
