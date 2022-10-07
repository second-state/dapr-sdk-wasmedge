#!/usr/bin/env bash

dapr run --app-id wasmedge-examples --app-protocol http --app-port 9005 --dapr-http-port 3503 --components-path ../config --log-level debug wasmedge dapr_examples.wasm

