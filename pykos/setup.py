# mypy: disable-error-code="import-untyped"
#!/usr/bin/env python
"""Setup script for the project."""

import os
import re
import subprocess
from typing import List

from setuptools import setup
from setuptools.command.build_py import build_py
from setuptools.command.egg_info import egg_info


class GenerateProtosMixin:
    """Mixin class to generate protos."""

    def generate_protos(self) -> None:
        """Generate proto files if Makefile exists."""
        if os.path.exists("Makefile"):
            subprocess.check_call(["make", "generate-proto"])


class BuildPyCommand(build_py, GenerateProtosMixin):
    """Custom build command to generate protos before building."""

    def run(self) -> None:
        """Run the build command."""
        self.generate_protos()
        super().run()


class EggInfoCommand(egg_info, GenerateProtosMixin):
    """Custom egg_info command to generate protos before creating egg-info."""

    def run(self) -> None:
        """Run the egg_info command."""
        self.generate_protos()
        super().run()


with open("README.md", "r", encoding="utf-8") as f:
    long_description: str = f.read()


with open("pykos/requirements.txt", "r", encoding="utf-8") as f:
    requirements: List[str] = f.read().splitlines()


with open("pykos/requirements-dev.txt", "r", encoding="utf-8") as f:
    requirements_dev: List[str] = f.read().splitlines()


with open("pykos/__init__.py", "r", encoding="utf-8") as fh:
    version_re = re.search(r"^__version__ = \"([^\"]*)\"", fh.read(), re.MULTILINE)
assert version_re is not None, "Could not find version in pykos/__init__.py"
version: str = version_re.group(1)


setup(
    name="pykos",
    version=version,
    description="The KOS command line interface",
    author="pykos Contributors",
    url="https://github.com/kscalelabs/kos",
    long_description=long_description,
    long_description_content_type="text/markdown",
    python_requires=">=3.8",
    install_requires=requirements,
    extras_require={"dev": requirements_dev},
    packages=["pykos", "kos_protos"],
    package_data={
        "pykos": ["py.typed"],
        "kos_protos": ["py.typed"],
    },
    include_package_data=True,
    entry_points={
        "console_scripts": [
            "pykos=pykos.cli:cli",
        ],
    },
    setup_requires=["grpcio-tools"],
    cmdclass={
        'build_py': BuildPyCommand,
        'egg_info': EggInfoCommand,
    },
)
