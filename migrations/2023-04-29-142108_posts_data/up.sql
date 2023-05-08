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
CREATE TABLE IF NOT EXISTS posts (
  id SERIAL PRIMARY KEY,
  user_id INTEGER REFERENCES users(id) NOT NULL,
  heading VARCHAR(200),
  sub_heading VARCHAR(200),
  caption VARCHAR(500),
  cooking_duration TIME,
  tags TEXT [],
  visuals TEXT [],
  ingredients TEXT [],
  steps TEXT [],
  likes_count INTEGER,
  comments_count INTEGER,
  created_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS likes (
  post_id INTEGER REFERENCES posts(id) ON DELETE CASCADE NOT NULL,
  user_id INTEGER REFERENCES users(id) ON DELETE CASCADE NOT NULL,
  created_at TIMESTAMP,
  PRIMARY KEY (post_id, user_id)
);

CREATE TABLE IF NOT EXISTS comments (
  id SERIAL PRIMARY KEY,
  post_id INTEGER REFERENCES posts(id) ON DELETE CASCADE NOT NULL,
  user_id INTEGER REFERENCES users(id) ON DELETE CASCADE NOT NULL,
  note VARCHAR(500),
  created_at TIMESTAMP
);

CREATE OR REPLACE FUNCTION likes_changed() RETURNS trigger AS $likes_changed$
	DECLARE
      len integer := 0;
   	BEGIN
    	IF (TG_OP = 'INSERT') THEN
    		SELECT likes_count INTO len FROM posts WHERE id = NEW.post_id; 
          len := len + 1;
          UPDATE posts SET likes_count = len WHERE id = NEW.post_id; 
      ELSIF (TG_OP = 'DELETE') THEN
    		SELECT likes_count INTO len FROM posts WHERE id = OLD.post_id; 
          len := len - 1;
          UPDATE posts SET likes_count = len WHERE id = OLD.post_id; 
		  END IF;
        RETURN NULL;
    END;
$likes_changed$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER like_added AFTER INSERT OR DELETE ON likes FOR EACH ROW EXECUTE FUNCTION likes_changed();

CREATE OR REPLACE FUNCTION comments_changed() RETURNS trigger AS $comments_changed$
	DECLARE
      len integer := 0;
   	BEGIN
    	IF (TG_OP = 'INSERT') THEN
    		SELECT comments_count INTO len FROM posts WHERE id = NEW.post_id; 
          len := len + 1;
          UPDATE posts SET comments_count = len WHERE id = NEW.post_id; 
      ELSIF (TG_OP = 'DELETE') THEN
    		SELECT comments_count INTO len FROM posts WHERE id = OLD.post_id; 
          len := len - 1;
          UPDATE posts SET comments_count = len WHERE id = OLD.post_id; 
		  END IF;
        RETURN NULL;
    END;
$comments_changed$ LANGUAGE plpgsql;
CREATE OR REPLACE TRIGGER comment_added AFTER INSERT OR DELETE ON comments FOR EACH ROW EXECUTE FUNCTION comments_changed();
