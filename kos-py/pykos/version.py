"""Version information for PyKOS."""

import re
from pathlib import Path

PYTHON_PATCH_VERSION = 1


def get_workspace_root() -> Path:
    """Get the workspace root directory."""
    # When running normally, use __file__
    if "__file__" in globals():
        return Path(__file__).parents[2]

    raise RuntimeError("Could not determine workspace root")


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
