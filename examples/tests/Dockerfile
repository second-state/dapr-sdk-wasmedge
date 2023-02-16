FROM python:3.7.1
RUN curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
ENV PATH="/root/.wasmedge/bin:${PATH}"
ADD dapr_examples.wasm /
CMD ["wasmedge", "--dir", ".:/", "/dapr_examples.wasm"]
