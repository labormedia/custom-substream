# Custom-Substream

You can follow the tutorial at [link]

## Requisites:
- Golang
- Rust
- Substreams (https://github.com/streamingfast/substreams)

## Run it :
```
git clone https://github.com/labormedia/custom-substream.git
cd custom-substream
substreams protogen ./substreams.yaml --exclude-paths="sf/ethereum,sf/substreams,google"
cargo build --target wasm32-unknown-unknown --release
```

Enjoy!
