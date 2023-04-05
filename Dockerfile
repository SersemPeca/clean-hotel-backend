# Build
FROM rust:latest as build

WORKDIR /usr/src/clean-hotel-backend

COPY . .

RUN cargo install diesel_cli --no-default-features --features sqlite
RUN ["cargo", "build", "--release"]
RUN ["diesel", "migration", "run"]

ARG API_PORT
ARG DATABASE_URL
ARG JWT_SECRET

ENV API_PORT=${API_PORT}
ENV DATABASE_URL=${DATABASE_URL}
ENV JWT_SECRET=${JWT_SECRET}

EXPOSE ${API_PORT}

CMD ["cargo", "run"]
