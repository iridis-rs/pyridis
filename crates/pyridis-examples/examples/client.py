from typing import Any, Dict
from pyridis_api import Node, Query, Inputs, Outputs, Queries, Queryables

import pyarrow as pa
import asyncio
import time

class MyClient(Node):
    ask_128: Query
    ask_64: Query

    def __init__(self):
        pass

    async def new(self, inputs: Inputs, outputs: Outputs, queries: Queries, queryables: Queryables, config: Dict[str, Any]):
        self.ask_128 = await queries.with_query("ask_128")
        self.ask_64 = await queries.with_query("ask_64")

    async def start(self):
        answer = await self.ask_128.query(pa.array([100]))
        print(answer.data[0])

        answer = await self.ask_128.query(pa.array([200]))
        print(answer.data[0])

        answer = await self.ask_64.query(pa.array([100]))
        print(answer.data[0])

        answer = await self.ask_64.query(pa.array([2]))
        print(answer.data[0])

def pyridis_node():
    return MyClient
