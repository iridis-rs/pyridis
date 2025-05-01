import pyarrow as pa

class ArrowMessage:
    @classmethod
    def from_arrow(cls, data: pa.Array) -> "ArrowMessage":
        pass
