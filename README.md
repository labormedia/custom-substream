# Custom-Substream

You can follow the tutorial at [link]

## Requisites:
- Golang
- Rust
- Substreams (https://github.com/streamingfast/substreams)

## Build it :
```
git clone https://github.com/labormedia/custom-substream.git
cd custom-substream
substreams protogen ./substreams.yaml --exclude-paths="sf/ethereum,sf/substreams,google"
cargo build --target wasm32-unknown-unknown --release
```

## Run it :
```
substreams run -e api-dev.streamingfast.io:443 substreams.yaml map_transfers --start-block 15627279 --stop-block +30
```

## Enjoy!
