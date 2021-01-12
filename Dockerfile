FROM jdrouet/rust-nightly:buster-slim as build-env
RUN apt-get update && apt-get install -y pkg-config libssl-dev g++
WORKDIR /app
ADD . /app
RUN cargo build --release
ENV LD_LIBRARY_PATH=/app/target/release/build/torch-sys-40d4b85b267a68ba/out/libtorch/libtorch/lib/
CMD ["/app/target/release/sentiment-api"]
