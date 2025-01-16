#!/bin/zsh
# Bump version number in both the Python and Rust packages, first checking
# that they match.

# Allows users to specify bumping major, minor or patch version.
# Defaults to patch.
if [[ $# -eq 0 ]]; then
    bump=patch
else
    bump=$1
    if [[ $bump != major && $bump != minor && $bump != patch ]]; then
        echo "!!! Invalid bump type: $bump !!!"
        exit 1
    fi
fi

package=$(basename $(pwd))
echo " ↪ Bumping version for package $package"

# Define the fixed paths for both version files
bash_script_file=$(dirname $0)
cargo_file="$bash_script_file/../Cargo.toml"
python_file="$bash_script_file/../kos-py/pykos/__init__.py"

# Check that both files exist
if [[ ! -f $cargo_file || ! -f $python_file ]]; then
    echo "!!! Could not find one or both version files !!!"
    echo "Expected files at: $cargo_file and $python_file"
    exit 1
fi

# Get versions from both files
cargo_version=$(grep '^version = ' $cargo_file | cut -d'"' -f2)
python_version=$(grep __version__ $python_file | cut -d\" -f2)

# Check if we found both versions
if [[ -z $cargo_version || -z $python_version ]]; then
    echo "!!! Could not extract version from one or both files !!!"
    exit 1
fi

# Check versions match
if [[ $cargo_version != $python_version ]]; then
    echo "!!! Version mismatch !!!"
    echo "Cargo version: $cargo_version"
    echo "Python version: $python_version"
    exit 1
fi

version=$cargo_version
echo " ↪ Current version is \033[1;31m$version\033[0m"

# Checks that version number is in the format x.y.z.
if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "!!! Invalid version number: $version !!!"
    exit 1
fi

# Bumps the version number.
major=$(echo $version | cut -d. -f1)
minor=$(echo $version | cut -d. -f2)
patch=$(echo $version | cut -d. -f3)
if [[ $bump == major ]]; then
    major=$((major + 1))
    minor=0
    patch=0
elif [[ $bump == minor ]]; then
    minor=$((minor + 1))
    patch=0
else
    patch=$((patch + 1))
fi
new_version="$major.$minor.$patch"
echo " ↪ New version is \033[1;32m$new_version\033[0m"

# Update both version files
tmp_cargo=$(mktemp)
tmp_python=$(mktemp)

awk -v old="version = \"$version\"" -v new="version = \"$new_version\"" '{ sub(old, new) } 1' $cargo_file > $tmp_cargo
awk -v old="__version__ = \"$version\"" -v new="__version__ = \"$new_version\"" '{ sub(old, new) } 1' $python_file > $tmp_python

mv $tmp_cargo $cargo_file
mv $tmp_python $python_file

# Commit both files
git add $cargo_file $python_file
git commit -m "Bump version to $new_version"
echo " ↪ Committed new version number"
