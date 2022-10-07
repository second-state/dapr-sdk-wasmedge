#!/usr/bin/env bash

dapr run --app-id echo-service --app-protocol http --app-port 9004 --dapr-http-port 3502 --components-path ../config --log-level debug wasmedge dapr_echo.wasm

