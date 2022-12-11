# login
#near login

# build & test
RUSTFLAGS='-C link-arg=-s' cargo build --manifest-path ./Cargo.toml --target wasm32-unknown-unknown --release
cp ./target/wasm32-unknown-unknown/release/*.wasm ./res/

# redeploy contracts
near deploy --accountId dev-1670785272066-17687747370547 \
  --wasmFile ./res/gastest.wasm