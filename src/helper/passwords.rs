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

use super::errors::InternalServerError;

use log::error;

pub(crate) fn hash(pass: &str) -> Result<String, InternalServerError> {
    argon2::hash_encoded(
        pass.as_bytes(),
        crate::CONFIG.salts.password.as_ref(),
        &argon2::Config::default(),
    )
    .map_err(|e| {
        error!("{:?}", e);
        InternalServerError
    })
}

pub(crate) fn verify(password: &str, hashed: &str) -> Result<bool, InternalServerError> {
    if hash(password)? == hashed {
        Ok(true)
    } else {
        Ok(false)
    }
}
