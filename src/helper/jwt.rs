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

use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header as JWTHeader, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

fn calc_exp(dur: Duration) -> i64 {
    (Utc::now() + dur).timestamp()
}

pub(crate) trait Claims: Sized + Serialize + DeserializeOwned {
    fn encode(&self) -> jsonwebtoken::errors::Result<String> {
        let key = EncodingKey::from_secret(crate::CONFIG.salts.jwt.as_ref());
        jsonwebtoken::encode(&JWTHeader::default(), self, &key)
    }

    fn decode(token: &str) -> jsonwebtoken::errors::Result<Self> {
        let key = DecodingKey::from_secret(crate::CONFIG.salts.jwt.as_ref());
        jsonwebtoken::decode::<Self>(token, &key, &Validation::default()).map(|t| t.claims)
    }

    fn default_exp_duration() -> i64;
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct SessionClaims {
    pub(crate) user_id: i32,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) avatar: String,
    pub(crate) jti: Uuid,
    pub(crate) exp: i64,
}

impl Claims for SessionClaims {
    fn default_exp_duration() -> i64 {
        calc_exp(Duration::seconds(60 * 60 * 24 * 7)) // 1 week
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AccountCreationClaims {
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) avatar: Option<String>,
    pub(crate) jti: Uuid,
    pub(crate) exp: i64,
}

impl Claims for AccountCreationClaims {
    fn default_exp_duration() -> i64 {
        calc_exp(Duration::seconds(60 * 60)) // 1 hour
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct PasswordChangeClaims {
    pub(crate) user_id: i32,
    pub(crate) password: String,
    pub(crate) jti: Uuid,
    pub(crate) exp: i64,
}

impl Claims for PasswordChangeClaims {
    fn default_exp_duration() -> i64 {
        calc_exp(Duration::seconds(60 * 60 * 24 * 7)) // 1 week
    }
}
