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

use super::views;
use actix_web::web;

pub(crate) fn config_with_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(views::create_post);
    cfg.service(views::update_post);
    cfg.service(views::list_posts);
    cfg.service(views::delete_post);

    cfg.service(views::add_like);
    cfg.service(views::delete_like);

    cfg.service(views::add_comment);
    cfg.service(views::delete_comment);
    cfg.service(views::get_comments);
}

pub(crate) fn config_without_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(views::get_post);
}
