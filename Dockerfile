FROM rust:latest as builder

RUN \
  curl -sL https://deb.nodesource.com/setup_12.x | bash - && \
  apt-get update && apt-get install nodejs && \
  cargo install wasm-pack && \
  rustup target add wasm32-unknown-unknown

RUN USER=root cargo new --lib anilist-toolbox

WORKDIR ./anilist-toolbox

COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release && rm src/*.rs

COPY ./package*.json ./
RUN npm install

COPY . ./

RUN npm run build

FROM nginx:alpine

RUN sed -i '97i application/wasm wasm;' /etc/nginx/mime.types
COPY docker/nginx.conf /etc/nginx/conf.d/default.conf

COPY --from=builder ./anilist-toolbox/dist /usr/share/nginx/html

ENTRYPOINT ["nginx"]
CMD ["-g", "daemon off;"]