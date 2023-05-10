# Food Book API

Food Book API is backend for Food Book which is a platform to share cooking recipies.

> You may check `api_doc.md` for api documentation

## Running with docker

* Create a directory named `static_dir`
* Copy `.dodocker-compose.yml` as `dodocker-compose.yml`. You may wish to change password of database.
* Copy `etc/.config.toml` as `etc/config.toml`. Your database url might look like `postgresql://postgres:<your password>@db:5432/postgres`. Also note that mention `static_dir` which we created in field `static_dir`. Further you will have to add your smtp server credientials.
* Run `docker compose build`
* Run `docker compose up`

## LICENSE

Food Book API is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Food Book API is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Food Book API. If not, see <https://www.gnu.org/licenses/>. 
