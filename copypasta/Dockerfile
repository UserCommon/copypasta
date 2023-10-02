# Builder stage
FROM rust AS build-env


COPY . /app

WORKDIR /app

RUN apt update && apt install lld clang -y

RUN cargo build --release

# run stage
FROM build-env as EXEC

COPY --from=build-env /app /app

WORKDIR /app

ENTRYPOINT ["./target/release/copypasta"]
