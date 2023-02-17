FROM python:3.7.1
RUN curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
ENV PATH="/root/.wasmedge/bin:${PATH}"

RUN wget -q https://raw.githubusercontent.com/dapr/cli/master/install/install.sh -O - | /bin/bash
RUN dapr init --slim

ADD dapr_echo.wasm /
ENTRYPOINT ["dapr"]
CMD ["run","--app-id", "echo-service", "--app-port", "9004", "--dapr-grpc-port", "50002", "--dapr-http-port","3502", "--log-level", "debug","--components-path","/opt/config", "wasmedge","dapr_echo.wasm"]
