RUST = cargo
SRCS = src/*.rs src/**/*.rs

FRONTEND_DIR = frontend/
OUTPUT = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].name')

DOCKER_TAG = $(OUTPUT):trixie

APP_NAME := $(OUTPUT)
VERSION ?= $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
BRANCH ?= $(shell git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
HASH ?= $(shell git rev-parse --short HEAD 2>/dev/null || echo "unknown")
BUILD_TIME := $(shell date -u +"%Y-%m-%dT%H:%M:%SZ")

RUST_ENV := APP_NAME=$(APP_NAME)			\
			APP_VERSION=$(VERSION)			\
            APP_BRANCH=$(BRANCH)			\
            APP_HASH=$(HASH)				\
            APP_BUILD_TIME=$(BUILD_TIME)

.PHONY: all build-frontend build run docker clean

all: build-frontend build

build-frontend: $(FRONTEND_DIR)
	@rm -rf public/
	@make -C $(FRONTEND_DIR)

build: $(SRCS)
	@$(RUST_ENV) $(RUST) build --release

run: ./target/release/$(OUTPUT)
	@bash -c target/release/$(OUTPUT)

docker:
	@docker build -t $(DOCKER_TAG) .

clean:
	@make -C $(FRONTEND_DIR) clean
	@$(RUST) clean
	@rm -rf public/
