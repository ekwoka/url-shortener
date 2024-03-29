# We do not need the Rust toolchain to run the binary!
FROM alpine:3.19 AS runtime
COPY /target/x86_64-unknown-linux-musl/release/url_shortener /usr/local/bin
COPY /config.yaml .
ENTRYPOINT ["/usr/local/bin/url_shortener"]
