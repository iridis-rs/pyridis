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
