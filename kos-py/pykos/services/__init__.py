"""KOS service clients."""

import asyncio
from abc import ABC
from functools import wraps
from inspect import getmembers, iscoroutinefunction
from typing import Any, Callable, TypeVar

T = TypeVar("T")


def add_sync_version(async_func: Callable[..., T]) -> Callable[..., T]:
    """Create a synchronous version of an async function."""

    @wraps(async_func)
    def sync_version(self: Any, *args: Any, **kwargs: Any) -> T:  # noqa: ANN401
        return asyncio.run(async_func(self, *args, **kwargs))

    # Rename the function to include _sync suffix
    sync_version.__name__ = f"{async_func.__name__}_sync"
    sync_version.__qualname__ = f"{async_func.__qualname__}_sync"
    if sync_version.__doc__:
        sync_version.__doc__ = f"Synchronous version of {async_func.__name__}()."

    return sync_version


class AsyncClientBase(ABC):
    """Base class for async gRPC clients that automatically adds sync versions of async methods."""

    def __init__(self) -> None:
        for name, method in getmembers(self.__class__, iscoroutinefunction):
            if name.startswith("__"):
                continue
            sync_method = add_sync_version(method)
            setattr(self.__class__, f"{name}_sync", sync_method)
