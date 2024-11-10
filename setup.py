# mypy: disable-error-code="import-untyped"
#!/usr/bin/env python
"""Setup script for the project."""

import re
import subprocess

from setuptools import find_packages, setup
from setuptools.command.build_ext import build_ext
from setuptools_rust import Binding, RustExtension

with open("README.md", "r", encoding="utf-8") as f:
    long_description: str = f.read()


with open("kos/requirements.txt", "r", encoding="utf-8") as f:
    requirements: list[str] = f.read().splitlines()


with open("kos/requirements-dev.txt", "r", encoding="utf-8") as f:
    requirements_dev: list[str] = f.read().splitlines()


with open("kos/__init__.py", "r", encoding="utf-8") as fh:
    version_re = re.search(r"^__version__ = \"([^\"]*)\"", fh.read(), re.MULTILINE)
assert version_re is not None, "Could not find version in kos/__init__.py"
version: str = version_re.group(1)


class RustBuildExt(build_ext):
    def run(self) -> None:
        # Run the stub generator
        subprocess.run(["cargo", "run", "--bin", "stub_gen"], check=True)
        # Call the original build_ext command
        super().run()


setup(
    name="pykos",
    version=version,
    description="The K-Scale Operating System",
    author="Benjamin Bolte",
    url="https://github.com/kscalelabs/kos",
    rust_extensions=[
        RustExtension(
            target="kos.bindings",
            path="kos/bindings/Cargo.toml",
            binding=Binding.PyO3,
        ),
    ],
    setup_requires=["setuptools-rust"],
    long_description=long_description,
    long_description_content_type="text/markdown",
    python_requires=">=3.11",
    install_requires=requirements,
    tests_require=requirements_dev,
    extras_require={"dev": requirements_dev},
    packages=find_packages(include=["kos"]),
    cmdclass={"build_ext": RustBuildExt},
)
