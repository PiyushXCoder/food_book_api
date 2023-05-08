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

use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::helper::{self, result::ResultExt};

use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{get, http::StatusCode, post, web, HttpResponse, Result as ActixResult};
use actix_web_grants::proc_macro::has_permissions;
use futures_util::StreamExt;
use serde_json::json;

#[get("/file/{name}")]
async fn get_file(name: web::Path<(String,)>) -> ActixResult<NamedFile> {
    let (name,) = name.into_inner();

    if name.starts_with(".") {
        return Err(crate::helper::errors::CustomMessageServerError {
            message: "File names doesn't start with dot".to_string(),
            status_code: StatusCode::BAD_REQUEST,
        }
        .into());
    }

    let mut path = crate::CONFIG.server.shared_dir.clone();
    path.push(&name);

    Ok(NamedFile::open(&path)?)
}

#[post("/file")]
#[has_permissions("LOGGEDIN")]
async fn save_file(mut payload: Multipart) -> ActixResult<HttpResponse> {
    let mut path = crate::CONFIG.server.shared_dir.clone();
    let mut filename: String = "".to_string();
    let mut buf: Option<BufWriter<File>> = None;
    while let Some(item) = payload.next().await {
        let mut field = item?;

        if let None = &buf {
            let mime = field
                .content_type()
                .ok_or(helper::errors::CustomMessageServerError {
                    message: "Require to specify proper content type".to_string(),
                    status_code: StatusCode::BAD_REQUEST,
                })
                .map(|m| m.to_string())?;

            match mime.as_str() {
                "video/mp4" => filename = format!("{}.mp4", helper::uuid()),
                "image/png" => filename = format!("{}.png", helper::uuid()),
                "image/jpeg" => filename = format!("{}.jpeg", helper::uuid()),
                _ => {
                    return Err(helper::errors::CustomMessageServerError {
                        message: "Require to specify proper content type".to_string(),
                        status_code: StatusCode::BAD_REQUEST,
                    }
                    .into());
                }
            };

            path.push(&filename.clone());
            let out_file = std::fs::File::create(&path)
                .map_err_log(|_| helper::errors::InternalServerError)?;
            buf = Some(std::io::BufWriter::new(out_file));
        }

        while let Some(chunk) = field.next().await {
            println!("CONTENT-TYPE: {:?}", field.content_type());
            if let Some(a) = &mut buf {
                a.write_all(&chunk?)
                    .map_err_log(|_| helper::errors::InternalServerError)?;
            }
        }
    }

    Ok(HttpResponse::Ok()
        .body(json!({ "filename": filename }).to_string())
        .into())
}
