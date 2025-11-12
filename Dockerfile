FROM rust:1.91

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

CMD ["rustoleum"]

