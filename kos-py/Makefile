# Makefile

define HELP_MESSAGE
K-Scale Operating System

# Installing

1. Create a new Conda environment: `conda create --name pykos python=3.11`
2. Activate the environment: `conda activate pykos`
3. Install the package: `make install-dev`

# Running Tests

1. Run autoformatting: `make format`
2. Run static checks: `make static-checks`
3. Run unit tests: `make test`

endef
export HELP_MESSAGE

all:
	@echo "$$HELP_MESSAGE"
.PHONY: all

# ------------------------ #
#    Protobuf Generation   #
# ------------------------ #

generate-proto:
	rm -rf kos_protos
	python -m grpc_tools.protoc \
		--python_out=. \
		--grpc_python_out=. \
		--mypy_out=. \
		--proto_path=../kos/proto/ \
		--proto_path=../kos/proto/googleapis \
		../kos/proto/kos/*.proto
	mkdir -p kos_protos
	mv kos/* kos_protos/
	rm -rf kos
	touch kos_protos/__init__.py

	# Fix imports in all generated files
	case "$$(uname)" in \
		Darwin) \
			find kos_protos -type f -name "*.py" -exec sed -i '' 's/from kos/from kos_protos/g' {} + && \
			find kos_protos -type f -name "*.py" -exec sed -i '' 's/import kos/import kos_protos/g' {} + \
			;; \
		*) \
			find kos_protos -type f -name "*.py" -exec sed -i 's/from kos/from kos_protos/g' {} + && \
			find kos_protos -type f -name "*.py" -exec sed -i 's/import kos/import kos_protos/g' {} + \
			;; \
	esac
.PHONY: generate-proto


# ------------------------ #
#          Build           #
# ------------------------ #

install-dev:
	@pip install --verbose -e '.[dev,webrtc]'
.PHONY: install-dev

# ------------------------ #
#        PyPI Build        #
# ------------------------ #

build-for-pypi:
	@pip install --verbose build wheel twine
	@python -m build --sdist --wheel --outdir dist/ .
	@twine upload dist/*
.PHONY: build-for-pypi

push-to-pypi: build-for-pypi
	@twine upload dist/*
.PHONY: push-to-pypi

# ------------------------ #
#       Static Checks      #
# ------------------------ #

py-files := $(shell find . -name '*.py' -not -path './kos_protos/*' -not -path './kos-py/kos_protos/*')

format:
	@black $(py-files)
	@ruff format $(py-files)
.PHONY: format

static-checks:
	@black --diff --check $(py-files)
	@ruff check $(py-files)
	@mypy --install-types --non-interactive $(py-files)
.PHONY: static-checks

# ------------------------ #
#        Unit tests        #
# ------------------------ #

test:
	python -m pytest
.PHONY: test
