# Sentiment API
Get sentiment analysis results from an API

## Usage
`sentiment-api`

Send POST requests to `:8000` with `{"data": ["text to analyze"]}`.

## Install
`cargo build --release` will output the binary to `target/release/sentiment-api`.

You also need it to be able to find the torch libs that are output. I just set it as an environment variable.

`LD_LIBRARY_PATH=target/release/build/torch-sys-40d4b85b267a68ba/out/libtorch/libtorch/lib/`