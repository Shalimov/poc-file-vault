FROM rust:1.48.0 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/app/file-vault-rust
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/app/file-vault-rust /usr/app/file-vault-rust

EXPOSE 3000

CMD ["/usr/app/file-vault-rust/target/release/file-vault-server"]