from typing import Any, Dict
from pyridis_api import Node, Input, Inputs, Outputs, Queries, Queryables

import asyncio

class MySink(Node):
    input: Input

    def __init__(self):
        pass

    async def new(self, inputs: Inputs, _outputs: Outputs, _queries: Queries, _queryables: Queryables, _config: Dict[str, Any]):
        print("Initiated")
        self.input = await inputs.with_input("in")

    async def start(self):
        print("Started!!")
        print("Stopped!!")

def pyridis_node():
    return MySink

# async def number():
#     return 42

# class Test:
#     input: int

#     def __init__(self) -> None:
#         pass

#     async def new(self) -> None:
#         self.input = await number()

# async def pyridis_node():
#     a = Test()

#     await a.new()

# # def pyridis_node():
# #     return Test()

# asyncio.run(pyridis_node())
