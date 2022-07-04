FROM rust:1.31

WORKDIR /usr/src/portfolio-back
COPY . .

RUN rustup default stable
RUN cargo build

CMD ["cargo", "run"]

