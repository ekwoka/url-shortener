# We do not need the Rust toolchain to run the binary!
FROM alpine:3.18 AS runtime
COPY /target/x86_64-unknown-linux-musl/release/url_shortener /usr/local/bin
ENTRYPOINT ["/usr/local/bin/url_shortener"]
