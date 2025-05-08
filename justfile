set quiet := true

fix:
    cargo fix --workspace --allow-dirty
    cargo clippy --fix --allow-dirty

clean:
    cargo clean
    rm -rf .venv

setup-uv:
    uv venv --python 3.12

home:
    echo $(cat .venv/pyvenv.cfg | grep -i home | cut -d '=' -f 2)/..

lib:
    echo $(just home)/lib

install-libraries:
    maturin develop --manifest-path crates/pyridis-api/Cargo.toml --uv
    maturin develop --manifest-path crates/pyridis-message/Cargo.toml --uv

setup-python: setup-uv install-libraries

build-plugin:
    cargo build -p pyridis-file-ext --features "cdylib"

io_runtime: build-plugin
    LD_LIBRARY_PATH=$(just lib) cargo run --example io_runtime

service_runtime: build-plugin
    LD_LIBRARY_PATH=$(just lib) cargo run --example service_runtime

enum_inherit:
    uv run crates/pyridis-examples/examples/enum_inherit.py
