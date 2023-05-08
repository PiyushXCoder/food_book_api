-- This file is part of Food Book API.
--
-- Food Book API is free software: you can redistribute it and/or modify it under the terms
-- of the GNU General Public License as published by the Free Software Foundation,
-- either version 3 of the License, or (at your option) any later version.
--
-- Food Book API is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
-- without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
-- PURPOSE. See the GNU General Public License for more details.
--
-- You should have received a copy of the GNU General Public License along with Food Book API.
-- If not, see <https://www.gnu.org/licenses/>. 

-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  first_name VARCHAR(200) NOT NULL,
  last_name VARCHAR(200),
  email VARCHAR(200) UNIQUE NOT NULL,
  avatar VARCHAR(300),
  password VARCHAR(512) NOT NULL,
  created_at TIMESTAMP
);
