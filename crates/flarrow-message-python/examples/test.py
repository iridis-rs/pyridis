from dataclasses import dataclass
from pyflarrow_message import ArrowMessage
from enum import Enum
import pyarrow as pa

@dataclass
class MyMessage(ArrowMessage):
    width: int

class Encoding(ArrowMessage, Enum):
    pass

MyMessage.from_arrow(pa.array([]))
