FROM rust

MAINTAINER Anthony

WORKDIR /app

COPY . /app

RUN cargo build
