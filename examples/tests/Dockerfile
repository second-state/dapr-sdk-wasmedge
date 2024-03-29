# syntax=docker/dockerfile:1

FROM --platform=$BUILDPLATFORM rust:1.64 AS buildbase
WORKDIR /src
RUN <<EOT bash
    set -ex
    apt-get update
    apt-get install -y \
        git \
        clang
    rustup target add wasm32-wasi
EOT
# This line installs WasmEdge including the AOT compiler
RUN curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash

FROM buildbase AS build
COPY Cargo.toml .
COPY src ./src
ENV EXAMPLE_DAPR_URL "http://echoapp-dapr"
# Build the Wasm binary
RUN --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/cache \
    --mount=type=cache,target=/usr/local/cargo/registry/index \
    cargo build --target wasm32-wasi --release

RUN /root/.wasmedge/bin/wasmedgec target/wasm32-wasi/release/dapr_examples.wasm dapr_examples.wasm

FROM scratch
ENTRYPOINT [ "dapr_examples.wasm" ]
COPY --link --from=build /src/dapr_examples.wasm /dapr_examples.wasm