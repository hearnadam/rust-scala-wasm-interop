# Run script to create components and workers before execution
# Example: ./setup-workers.sh "001"

VERSION=$1 # Example: 001

# Build our Rust components first
cd ../rust

# 1) The Http Client: Add the component and create an idle worker 
cd http-client
cargo component build --release
golem-cli component add\
  --component-name "http-client-$VERSION"\
  ./target/wasm32-wasi/release/http_client.wasm
golem-cli worker add --component-name "http-client-$VERSION" -w "example-http-client-$VERSION"
cd ..


# 2) The Stub: Add the component and create an idle worker
cd http-client-stub
golem-cli component add\
  --component-name "http-client-stub-$VERSION"\
  http-client-stub.wasm
golem-cli worker add --component-name "http-client-stub-$VERSION" -w "example-http-client-stub-$VERSION"
cd ..

# Build our Scala components

# TODO: We build all the scala things here, then profit.

# Create a symbolic link to the wit directory for Scala to reference when building it's components
mkdir -p ../scala/wit/deps/
ln -s ../rust/http-client/wit/http-client.wit ../scala/wit/deps/http-client.wit
ln -s ../rust/http-client-stub/wit/_stub.wit ../scala/wit/deps/_stub.wit

