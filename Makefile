COMPONENTS = cli db server

define COMPONENT_RULES
test_$(1):
	cargo test --manifest-path=$(1)/Cargo.toml --verbose
.PHONY test: test_$(1)
endef

all:
.PHONY: all

test:
.PHONY: test

travis:
	cd server && travis-cargo coveralls --no-sudo --verify
	cargo doc --manifest-path=cli/Cargo.toml --verbose
	cd cli && travis-cargo doc-upload --branch=develop
.PHONY: travis

$(foreach component,$(COMPONENTS),$(eval $(call COMPONENT_RULES,$(component))))
