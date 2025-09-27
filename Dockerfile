ARG RUST_VERSION=1.87.0
ARG APP_NAME=loxosceles

FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app

RUN apt-get update && \
    apt-get install --no-install-recommends -y pkg-config openssl libssl-dev && \
    rm -rf /var/lib/apt/lists/*

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --release
cp ./target/release/$APP_NAME /bin/$APP_NAME
EOF

FROM debian:bullseye-slim AS final
ARG APP_NAME

RUN apt-get update && \
    apt-get install --no-install-recommends -y ca-certificates libssl1.1 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /bin/$APP_NAME /bin/
CMD ["/bin/loxosceles"]
