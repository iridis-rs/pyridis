from typing import Any, Dict
from pyridis_api import Node, Output, Inputs, Outputs, Queries, Queryables

import pyarrow as pa
import asyncio
import time

class MySource(Node):
    output: Output

    def __init__(self):
        pass

    async def new(self, _inputs: Inputs, outputs: Outputs, _queries: Queries, _queryables: Queryables, _config: Dict[str, Any]):
        self.output = await outputs.with_output("out")

    async def start(self):
        while True:
            try:
                await asyncio.sleep(1)
                await self.output.send(pa.array(["tick"]))
            except:
                break

def pyridis_node():
    return MySource
