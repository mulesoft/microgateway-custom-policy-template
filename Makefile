TARGET         := wasm32-wasi
TARGET_DIR     := target/$(TARGET)/release
NAME           := {{ crate_name }}
DEFINITION_NAME := $(shell anypoint-cli-v4 pdk policy-project definition get gcl-metadata-name)
DEFINITION_GCL_PATH := $(shell anypoint-cli-v4 pdk policy-project locate-gcl definition)
ASSET_VERSION := $(shell cargo anypoint get-version)

.phony: build-asset-files
build-asset-files: $(DEFINITION_GCL)
	@anypoint-cli-v4 pdk policy-project build-asset-files --version $(ASSET_VERSION)
	@cargo anypoint config-gen -p -m $(DEFINITION_GCL_PATH) -o src/generated/config.rs

.phony: build
build: build-asset-files
	@cargo build --target $(TARGET) --release
	@cp $(DEFINITION_GCL_PATH) $(TARGET_DIR)/$(NAME)_definition.yaml
	@cargo anypoint gcl-gen -d $(DEFINITION_NAME) -w $(TARGET_DIR)/$(NAME).wasm -o $(TARGET_DIR)/$(NAME)_implementation.yaml

.phony: deploy
deploy: build
	cp $(TARGET_DIR)/$(NAME).yaml test/config/custom-policies/$(NAME).yaml

.phony: publish
publish: build
	anypoint-cli-v4 pdk policy-project publish --binaryDir $(TARGET_DIR)

.phony: release
release: build
	anypoint-cli-v4 pdk policy-project release --binaryDir $(TARGET_DIR)
