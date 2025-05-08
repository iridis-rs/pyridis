from typing import Any, Dict

import pyarrow as pa
import asyncio
import time

from pyridis_api import Node, Output, Inputs, Outputs, Queries, Queryables

class MySource(Node):
    output: Output
    frequency: float

    def __init__(self):
        self.frequency = 1.0

    async def new(self, inputs: Inputs, outputs: Outputs, queries: Queries, queryables: Queryables, config: Dict[str, Any]):
        self.output = await outputs.with_output("out")
        self.frequency = float(config["frequency"]) if "frequency" in config else 1.0

    async def start(self):
        while True:
            try:
                await asyncio.sleep(1.0 / self.frequency)
                await self.output.send(pa.array(["tick"]))
            except:
                break

def pyridis_node():
    return MySource
