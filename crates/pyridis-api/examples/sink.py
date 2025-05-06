from typing import Any, Dict
from pyridis_api import Node, Input, Inputs, Outputs, Queries, Queryables

import asyncio
import time

class MySink(Node):
    input: Input

    def __init__(self):
        pass

    async def new(self, inputs: Inputs, _outputs: Outputs, _queries: Queries, _queryables: Queryables, _config: Dict[str, Any]):
        self.input = await inputs.with_input("in")

    async def start(self):
        while True:
            try:
                message = await self.input.recv()
                print(message.data)
            except:
                break

def pyridis_node():
    return MySink
