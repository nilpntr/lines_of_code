.PHONY: all
all: build run

.PHONY: release
release: build-prod release-prod cleanup-prod

.PHONY: build
build:
	cargo build

.PHONY: run
run:
	./target/debug/lines_of_code --file-extension=.rs --path=~/RustProjects/lines_of_code

.PHONY: build-prod
build-prod:
	cargo build --release

.PHONY: release-prod
release-prod:
	mkdir -p ~/.clipkg && mv target/release/lines_of_code ~/.clipkg && chmod +x ~/.clipkg/lines_of_code

.PHONY: cleanup-prod
cleanup-prod:
	rm -rf target/release