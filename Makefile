CARGO = $(shell which cargo)

.PHONY: build
build:
	$(CARGO) build

.PHONY: clean
clean:
	$(CARGO) clean

.PHONY: run
run:
	$(CARGO) run

.PHONY: lint
lint:
	$(CARGO) fmt -- --check
	$(CARGO) clippy

.PHONY: apply-format
apply-format:
	$(CARGO) fmt

.PHONY: test
test:
	$(CARGO) test
