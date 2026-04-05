PROJECT_NAME := genesis
PROJECT_PACKAGE := $(PROJECT_NAME)
PROJECT_CAP := $(shell echo $(PROJECT_NAME) | tr '[:lower:]' '[:upper:]')
CURRENT_VERSION := $(shell grep '^version = ' Cargo.toml | sed -E 's/version = "(.*)"/\1/')
LATEST_TAG ?= $(shell git describe --tags --abbrev=0 2>/dev/null)
TOP_DIR := $(CURDIR)
BUILD_DIR := $(TOP_DIR)/target

ifeq ($(PROJECT_NAME),)
$(error Error: project name not found in Cargo.toml)
endif

$(info ------------------------------------------)
$(info Project: $(PROJECT_NAME))
$(info Version: $(CURRENT_VERSION))
$(info ------------------------------------------)

.PHONY: build b compile c run r test t help h clean docs release

SHELL := /bin/bash

NIXGL_CMD := $(shell \
	if command -v nixGLNvidia >/dev/null 2>&1; then \
		echo nixGLNvidia; \
	else \
		compgen -c | grep '^nixGLNvidia-' | head -n1; \
	fi \
)

ifeq ($(strip $(NIXGL_CMD)),)
CARGO_CMD := cargo
else
CARGO_CMD := $(NIXGL_CMD) cargo
endif

build:
	@$(CARGO_CMD) build -p $(PROJECT_PACKAGE) --bin $(PROJECT_NAME)
	@if [ -d assets ]; then rm -rf $(BUILD_DIR)/debug/assets && cp -r assets $(BUILD_DIR)/debug/assets; fi

b: build

compile:
	@cargo clean
	@$(MAKE) build

c: compile

run: build
	@$(CARGO_CMD) run -p $(PROJECT_PACKAGE) --bin $(PROJECT_NAME)

r: run

test:
	@$(CARGO_CMD) test --workspace

t: test

help:
	@echo
	@echo "Usage: make [target]"
	@echo
	@echo "Detected GPU wrapper: $(if $(strip $(NIXGL_CMD)),$(NIXGL_CMD),none)"
	@echo "Cargo command: $(CARGO_CMD)"
	@echo
	@echo "Available targets:"
	@echo "  build        Build project"
	@echo "  compile      Clean and rebuild project"
	@echo "  run          Run the main executable"
	@echo "  test         Run tests"
	@echo "  release      Create a new release (TYPE=patch|minor|major)"
	@echo

h: help

TYPE ?= patch
HAS_REL := $(shell command -v git-rel 2>/dev/null)

release:
	@if [ -z "$(HAS_REL)" ]; then \
		echo "git-rel is not installed. Please install it first."; \
		exit 1; \
	fi
	@if [ -z "$(TYPE)" ]; then \
		echo "Release type not specified. Use 'make release TYPE=[patch|minor|major|m.m.p]'"; \
		exit 1; \
	fi
	@git rel $(TYPE)
