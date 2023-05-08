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

use log::error;
use std::{error::Error, panic::Location};

pub(crate) trait ResultExt<T, E> {
    fn map_err_log<F, O: FnOnce(E) -> F>(self, op: O) -> Result<T, F>;
}

impl<T, E: Error> ResultExt<T, E> for Result<T, E> {
    #[track_caller]
    fn map_err_log<F, O: FnOnce(E) -> F>(self, op: O) -> Result<T, F> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                error!("{}\n{:?}", Location::caller(), e);
                Err(op(e))
            }
        }
    }
}
