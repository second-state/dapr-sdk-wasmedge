FROM python:3.7.1
RUN curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash

RUN wget -q https://raw.githubusercontent.com/dapr/cli/master/install/install.sh -O - | /bin/bash
ARG DAPR_BUILD_DIR
COPY $DAPR_BUILD_DIR /opt/dapr
ENV PATH="/root/.wasmedge/bin:/opt/dapr/:${PATH}"
RUN dapr init --slim

ADD dapr_echo.wasm /
RUN mkdir /opt/config
COPY .. /opt/config
ENTRYPOINT ["dapr"]
CMD ["run","--app-id", "echoapp", "--app-port", "9004", "--dapr-grpc-port", "50002", "--dapr-http-port","3502", "--log-level", "debug","--components-path","/opt/config", "wasmedge","dapr_echo.wasm"]
