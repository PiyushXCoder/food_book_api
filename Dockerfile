# This file is part of Food Book API.
# 
# Food Book API is free software: you can redistribute it and/or modify it under the terms
# of the GNU General Public License as published by the Free Software Foundation,
# either version 3 of the License, or (at your option) any later version.
# 
# Food Book API is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
# without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
# PURPOSE. See the GNU General Public License for more details.
# 
# You should have received a copy of the GNU General Public License along with Food Book API.
# If not, see <https://www.gnu.org/licenses/>. 

FROM rust:1-alpine3.16

ENV APP=food_book_api
ENV PORT=8081

RUN cargo search --limit 0 && \
    apk upgrade --update-cache --available && \
    apk add musl-dev && \
    apk add pkgconfig && \
    apk add openssl-dev && \
    apk add libpq-dev && \
    cargo install diesel_cli --no-default-features --features postgres && \
    rm -rf /var/cache/apk/* && \
    mkdir -pv /app/${APP}/etc

WORKDIR /app/${APP}
COPY . .

RUN RUSTFLAGS="-C target-feature=-crt-static" cargo build --release && \
    cp target/release/${APP} . && \
    cargo clean && \
    rm -rf /usr/local/rustup/ /usr/local/cargo/

EXPOSE ${PORT}/tcp

CMD ./${APP} --config-file ./etc/config.toml
