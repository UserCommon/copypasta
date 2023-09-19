FROM rust

COPY . /app
WORKDIR /app

CMD ["cargo", "r"]