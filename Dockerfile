FROM rust:slim-bullseye

WORKDIR /pwd

RUN apt-get update
RUN apt-get install --no-install-recommends -y libinput-dev

# now fetch dependencies
RUN mkdir /pwd/src && touch /pwd/src/lib.rs
COPY ./Cargo.lock /pwd/Cargo.lock
COPY ./Cargo.toml /pwd/Cargo.toml
RUN cargo fetch
RUN rm -r /pwd/