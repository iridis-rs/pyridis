# pyridis

`pyridis` is the API and plugin to support python scripts inside [`iridis`](https://github.com/iridis-rs/iridis)

It consists in two main APIs:

- `pyridis-api`: the primary API used to implement each node in the dataflow graph.
- `pyridis-message`: the secondary API used to implement messages to be passed between nodes

The main part of this project is then the `PythonFileExtPlugin`, which is a plugin compiled as a `cdylib` that must be passed
to the `iridis` runtime in order to be able to load `.py` files as a node.

## Usage

In a single `.py` file, you can define a `node` like this:

```python
from typing import Any, Dict

import asyncio
import time

from pyridis_api import Node, Input, Inputs, Outputs, Queries, Queryables

class MySink(Node):
    input: Input

    def __init__(self):
        pass

    async def new(self, inputs: Inputs, outputs: Outputs, queries: Queries, queryables: Queryables, config: Dict[str, Any]):
        self.input = await inputs.with_input("in")

    async def start(self):
        while True:
            try:
                message = await self.input.recv()
                print(message.data[0])
            except:
                break

def pyridis_node():
    return MySink
```

Then you must load the `PythonFileExtPlugin` to your `rust` `iridis` runtime:

```rust
let runtime = Runtime::new(
    async |file_ext: &mut FileExtManagerBuilder, _url_scheme: &mut UrlSchemeManagerBuilder| {
        file_ext
            .load_dynamically_linked_plugin(PathBuf::from("/path/to/libpyridis_file_ext.dylib"))
            .await?;

        Ok(())
    },
)
.await?;
```

**Note:** it's also possible to load this plugin statically with `.load_statically_linked_plugin::<PythonFileExtPlugin>()`

Finally you can load your nodes into the runtime just like `rust` nodes:

```rust
runtime
    .run(flows, async move |loader: &mut NodeLoader| {
        loader
            .load_url(
                Url::parse("file:///path/to/script.py")?,
                source,
                serde_yml::from_str("frequency: 1.0")?,
            )
            .await?;

        loader
            .load_url(
                Url::parse("file:///path/to/other_script.py")?,
                sink,
                serde_yml::from_str("")?,
            )
            .await?;
        Ok(())
    })
    .await
```

Before you try to run anything, you must have activated a python virtual environment. Tests have been made with `uv` and it works just fine:

```bash
uv venv --python 3.12 # the version must match the one used to compile the 'libpyridis_file_ext` in case of dynamic linking
source .venv/bin/activate # or 'activate.fish', or 'activate.ps1' ...
```

Then you will need the `api`:

```bash
uv pip install pyridis-api pyridis-message pyarrow numpy
```

And finally you can build your application:

```bash
cargo build -p name-of-your-crate
```

**However**, the generated executable will not be able to find the correct `python` libs of the environment by default. You will need to tweak the
`LD_LIBRARY_PATH`:

```bash
source .venv/bin/activate
export LD_LIBRARY_PATH=$(echo $(cat .venv/pyvenv.cfg | grep -i home | cut -d '=' -f 2)/..) # adjust to reflect the location of your .venv
cargo run -p name-of-your-crate
```

For a complete example of a project with multiple nodes—see [pyridis-benchmark](https://github.com/iridis-rs/pyridis-benchmark).

## Examples

Multiple examples can be found in [this directory](crates/pyridis-examples) and can be launched with `just`:

### Example of message definitions

```bash
just enum_inherit
```

### Example of applications

```bash
just io_runtime
just service_runtime
```

## Rust

For now it's only possible to interact with an `iridis` runtime with `rust`. See [iridis](https://github.com/iridis-rs/iridis) for a detailed description of the project

## Benchmark

See [pyridis-benchmark](https://github.com/iridis-rs/pyridis-benchmark) for a detailed description of the benchmark.

<div align="center">
  <img src="https://raw.githubusercontent.com/iridis-rs/pyridis-benchmark/main/bench/benchmark_latency.svg" alt="Benchmark Latency">
</div>
