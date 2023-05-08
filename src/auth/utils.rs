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

use crate::helper;
use crate::helper::result::ResultExt;

use actix_web::Result as ActixResult;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub(crate) fn send_mail(to_address: &str, subject: &str, message: &str) -> ActixResult<()> {
    let from_address = &crate::CONFIG.smtp.from_address;
    let smtp_server = &crate::CONFIG.smtp.server;
    let smtp_username = &crate::CONFIG.smtp.username;
    let smtp_password = &crate::CONFIG.smtp.password;

    let email = Message::builder()
        .from(from_address.parse().unwrap())
        .to(to_address.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(message.to_string())
        .unwrap();

    let creds = Credentials::new(smtp_username.to_owned(), smtp_password.to_owned());

    let mailer = SmtpTransport::relay(smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    mailer
        .send(&email)
        .map_err_log(|_| helper::errors::InternalServerError.into())
        .map(|_| ())
}
