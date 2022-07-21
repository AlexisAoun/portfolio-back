FROM rust:1.31

ENV ROCKET_ADDRESS=0.0.0.0

WORKDIR /usr/src/portfolio-back
COPY . .

RUN rustup default stable
RUN cargo build

EXPOSE 8000

CMD ["cargo", "run"]

