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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Message {
    pub(crate) message: String,
}

impl Message {
    pub(crate) fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl ToString for Message {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ErrorMessage {
    pub(crate) message: String,
}

impl ErrorMessage {
    pub(crate) fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub(crate) fn new_internal_server() -> Self {
        Self {
            message: "Internal Server Error".to_string(),
        }
    }
}

impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
