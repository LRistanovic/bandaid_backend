FROM rust:latest

WORKDIR /usr/src/bandaid_api
COPY . .

RUN cargo install --path .

CMD ["bandaid_api"]