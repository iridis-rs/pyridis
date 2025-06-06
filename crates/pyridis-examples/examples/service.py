from typing import Any, Dict
from pyridis_api import Node, PyDataflowMessage, Queryable, Inputs, Outputs, Queries, Queryables

import pyarrow as pa
import asyncio
import time

class MyService(Node):
    compare_to_128: Queryable
    compare_to_64: Queryable

    def __init__(self):
        pass

    async def new(self, inputs: Inputs, outputs: Outputs, queries: Queries, queryables: Queryables, config: Dict[str, Any]):
        self.compare_to_128 = await queryables.with_queryable("compare_to_128")
        self.compare_to_64 = await queryables.with_queryable("compare_to_64")

    def func_compare_to_128(self, message: PyDataflowMessage) -> pa.Array:
        if message.data.to_pylist()[0] > 128:
            return pa.array([f"{message.data[0]} is greater than 128"])
        else:
            return pa.array([f"{message.data[0]} is less than or equal to 128"])

    def func_compare_to_64(self, message: PyDataflowMessage) -> pa.Array:
        if message.data.to_pylist()[0] > 64:
            return pa.array([f"{message.data[0]} is greater than 64"])
        else:
            return pa.array([f"{message.data[0]} is less than or equal to 64"])

    async def service_128(self):
        while True:
            try:
                await self.compare_to_128.on_query(self.func_compare_to_128)
            except:
                break

    async def service_64(self):
        while True:
            try:
                await self.compare_to_64.on_query(self.func_compare_to_64)
            except:
                break

    async def start(self):
        t1 = asyncio.create_task(self.service_128())
        t2 = asyncio.create_task(self.service_64())

        await t1
        await t2

def pyridis_node():
    return MyService
