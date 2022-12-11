# login
#near login

# build & test
RUSTFLAGS='-C link-arg=-s' cargo build --manifest-path ./Cargo.toml --target wasm32-unknown-unknown --release
cp ./target/wasm32-unknown-unknown/release/*.wasm ./res/

# redeploy contracts
near dev-deploy \
  --wasmFile ./res/gastest.wasm \
  --initFunction 'new' \
  --initArgs '{}'


