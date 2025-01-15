"""KOS Python client."""

from pykos.client import KOS
from pykos.version import __version__

from . import services

__all__ = ["KOS", "__version__"]
