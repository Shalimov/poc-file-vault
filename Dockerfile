FROM rust:1.48.0 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/file-vault-rust
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/file-vault-rust /usr/local/bin/file-vault-rust

EXPOSE 3000

CMD ["file-vault-rust"]