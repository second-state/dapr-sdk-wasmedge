# dapr-sdk-wasi

A Dapr SDK in Rust. dapr-sdk-wasi is designed to run in WasmEdge application.

[Introduction](#introduction) | [How it works](#how-it-works) | [Usage](#usage) | [Contribution](#contribution)

## Introduction

> This is an experimental project now.

WebAssembly is an ideal environment to [run microservices](https://medium.com/wasm/cloud-native-webassembly-in-service-mesh-b19e3a96ccf8) with high performance and security. 

Unfortunately, the official Dapr Rust SDK couldn't be compiled into a WASI program, so we rewrote the Dapr sdk in reqwest(a popular HTTP client for Rust) to make it could run in WasmEdge. 

Now the dapr-sdk-wasi supports service invocation, state management, secrets, and health APIs.

| Dapr API               | Status     |
|------------------------|------------|
| Service invocation API | ✅          |
| State management API   | ✅          |
| Pub/sub API            | To be done |
| Bindings API           | To be done |
| Actors API             | To be done |
| Secrets API            | ✅          |
| Configuration API      | To be done |
| Distributed Lock API   | To be done |
| Health API             | ✅          |
| Metadata API           | To be done |


## How it works
There are two parts in this repo. One is the SDK, and the other is the example to demonstrate how to use dapr-wasi-sdk to acess Dapr API.

* The whole project is the SDK itself, which is responsible for the web services provided by Dapr (ie, Dapr API).
* `examples/echo` is a sidecar demo built by the dapr-sdk-wasi, which accesses the service invocation, state management, secrets, and health APIs
* `examples/test` is to verify the web service in `examples/echo`

If you want to run the `echo` example, follow the following instructions.

### Install Rust and add Wasi target for the Rust compiler

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-wasi
```
### Install WasmEdge

```
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
```
### Install and init Dapr

```
wget -q https://raw.githubusercontent.com/dapr/cli/master/install/install.sh -O - | /bin/bash
dapr init
```

### Run `echo` examples
```
cd examples/echo
// compile the rust code to wasm 
cargo build --target wasm32-wasi --release
// run the SDK in AOT model
wasmedgec target/wasm32-wasi/release/dapr_echo.wasm dapr_echo.wasm
// Access Dapr API
nohup dapr run --app-id echo-service --app-protocol http --app-port 9004 --dapr-http-port 3502 --components-path ../config --log-level debug wasmedge dapr_echo.wasm > server.log 2>&1 &
// return the server log
ℹ️  Starting Dapr with id echo-service. HTTP Port: 3502. gRPC Port: 44517
time="2022-10-07T22:00:17.732779744Z" level=info msg="starting Dapr Runtime -- version 1.8.4 -- commit 18575823c74318c811d6cd6f57ffac76d5debe93" app_id=echo-service instance=fv-az186-200 scope=dapr.runtime type=log ver=1.8.4
time="2022-10-07T22:00:17.732813444Z" level=info msg="log level set to: debug" app_id=echo-service instance=fv-az186-200 scope=dapr.runtime type=log ver=1.8.4
···
```

## Usage

This project allows developers to run a complete microservice sidecar in WasmEdge, without any host applications and [Linux containers](https://wasmedge.org/wasm_linux_container/). See an example [here](https://github.com/second-state/dapr-wasm).

The following image shows how dapr-wasi-sdk works with the microservice sidecar and the Dapr sidecar. dapr-wasi-sdk is responsible for the communication with Dapr sidecar.

![](dapr-sdk-wasi.png)



## Contribution

Any feedback is appreciated. If you have any questions or suggestions, please raise an issue or create a PR and let us know.

If you prefer chatting in real-time, join our [Discord server](https://discord.gg/U4B5sFTkFc).

## Tech stacks used in this project
* Tokio
* Reqwest-wasi
* Dapr
* WasmEdge
