TARGET         		:= wasm32-wasi
TARGET_DIR     		:= target/$(TARGET)/release
NAME           		:= {{ crate_name }}
CARGO_ANYPOINT 		:= cargo-anypoint
DEFINITION_NAME     = $(shell anypoint-cli-v4 pdk policy-project definition get gcl-metadata-name)
DEFINITION_GCL_PATH = $(shell anypoint-cli-v4 pdk policy-project locate-gcl definition)
ASSET_VERSION       = $(shell cargo anypoint get-version)
CRATE_NAME          = $(shell cargo anypoint get-name)
OAUTH_TOKEN         = $(shell anypoint-cli-v4 conf token | grep -o '"token": *"[^"]*"' | cut -d '"' -f 4)

.phony: setup
setup: login install-cargo-anypoint
	cargo +nightly fetch -Z registry-auth

.phony: build
build: build-asset-files
	@cargo build --target $(TARGET) --release
	@cp $(DEFINITION_GCL_PATH) $(TARGET_DIR)/$(NAME)_definition.yaml
	@cargo anypoint gcl-gen -d $(DEFINITION_NAME) -w $(TARGET_DIR)/$(NAME).wasm -o $(TARGET_DIR)/$(NAME)_implementation.yaml

.phony: run
run: build
	@anypoint-cli-v4 pdk log -t "warn" -m "Remember to update the config values in test/config/api.yaml file for the policy configuration"
	@anypoint-cli-v4 pdk patch-gcl -f test/config/api.yaml -p "spec.policies[0].policyRef.name" -v "$(DEFINITION_NAME)-impl"
	cp $(TARGET_DIR)/$(NAME)_implementation.yaml test/config/custom-policies/$(NAME)_implementation.yaml
	cp $(TARGET_DIR)/$(NAME)_definition.yaml test/config/custom-policies/$(NAME)_definition.yaml
	docker compose -f ./test/docker-compose.yaml up

.phony: publish
publish: build
	anypoint-cli-v4 pdk policy-project publish --binaryPath $(TARGET_DIR)/$(NAME).wasm

.phony: release
release: build
	anypoint-cli-v4 pdk policy-project release --binaryPath $(TARGET_DIR)/$(NAME).wasm

.phony: build-asset-files
build-asset-files: $(DEFINITION_GCL)
	@anypoint-cli-v4 pdk policy-project build-asset-files --version $(ASSET_VERSION) --asset-id $(CRATE_NAME)
	@cargo anypoint config-gen -p -m $(DEFINITION_GCL_PATH) -o src/generated/config.rs

.phony: login
login:
	cargo login --registry anypoint $(OAUTH_TOKEN)

.phony: install-cargo-anypoint
install-cargo-anypoint:
	@if ! cargo install --list | grep -q '^$(CARGO_ANYPOINT) '; then \
		echo "Installing $(CARGO_ANYPOINT)"; \
		cargo +nightly install cargo-anypoint --registry anypoint -Z registry-auth --config .cargo/config.toml; \
	fi
