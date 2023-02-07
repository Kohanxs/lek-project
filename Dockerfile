FROM rustlang/rust:nightly-slim as builder


WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y libpq-dev
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install libpq5 -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/lek-project /usr/local/bin/lek-project
EXPOSE 8000
ENTRYPOINT ["lek-project"]