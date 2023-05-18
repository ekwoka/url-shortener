# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
COPY /target/x86_64-unknown-linux-gnu/release/url_shortener /usr/local/bin
ENTRYPOINT ["/usr/local/bin/url_shortener"]
