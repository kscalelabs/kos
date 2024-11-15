"""Defines PyTest configuration for the project."""

import random
from typing import List

import pytest
from _pytest.python import Function


@pytest.fixture(autouse=True)
def set_random_seed() -> None:
    random.seed(1337)


def pytest_collection_modifyitems(items: List[Function]) -> None:
    items.sort(key=lambda x: x.get_closest_marker("slow") is not None)
