"""Version information for PyKOS."""

import re
from pathlib import Path

PYTHON_PATCH_VERSION = 3


def get_workspace_root() -> Path:
    """Get the workspace root directory."""
    # Look for Cargo.toml in the same directory as the package
    cargo_toml = Path(__file__).parent / "Cargo.toml"
    if cargo_toml.exists():
        return cargo_toml.parent

    raise RuntimeError("Could not find Cargo.toml")


def get_version() -> str:
    """Get version from workspace Cargo.toml."""
    try:
        cargo_toml = get_workspace_root() / "Cargo.toml"
        with open(cargo_toml, "r") as f:
            cargo_content = f.read()

        # Extract major.minor version from Cargo.toml
        match = re.search(r'version\s*=\s*"(\d+\.\d+)', cargo_content)
        if match:
            return f"{match.group(1)}.{PYTHON_PATCH_VERSION}"  # Add Python patch version
    except (FileNotFoundError, IndexError):
        print("Could not find version in Cargo.toml")
        raise RuntimeError("Could not find version in Cargo.toml")

    return "0.0.1"


__version__ = get_version()
