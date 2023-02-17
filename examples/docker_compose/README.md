

In a production environment, we need to run the Wasm app and Dapr in two separate containers. Specially, we want to run this in [Docker Compose](https://docs.docker.com/compose/). Refer [this](https://github.com/WasmEdge/docs/blob/main/docs/develop/getting-started/quick_start_docker.md) link to get started with WasmEdge and Docker
.
```
Compose is a tool for defining and running multi-container Docker applications. With Compose, you use a YAML file to configure your application’s services. Then, with a single command, you create and start all the services from your configuration.
```

Check if you can run containerd using the following command:

```sh
docker run --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm secondstate/rust-example-hello:latest
```

If the above command is successful, to start the docker containers use the following command:

It is necessary to change the path of secrets:

```sh
sed -i 's/\..\/config\/secrets.json/\/opt\/config\/secrets.json/g' ../config/local-secret-store.yaml
```

After changing the secrets path:

```sh
docker-compose up
```

To stop the containers:

```sh
docker-compose down
```

Output:
[![asciicast](https://asciinema.org/a/560561.svg)](https://asciinema.org/a/560561)