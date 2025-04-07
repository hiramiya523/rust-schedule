FROM rust:1.86 as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/release/temp-test /usr/local/bin/app
EXPOSE 3000
CMD ["app"] 