from enum import Enum
from loro import loro as _loro
from loro.loro import *

class ContainerType(Enum):
    Map = "map"
    List = "list"
    Tree = "tree"
    Counter = "counter"
    Text = "text"

__doc__ = _loro.__doc__
if hasattr(_loro, "__all__"):
    __all__ = _loro.__all__
