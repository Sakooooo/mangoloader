PROJDIR := $(shell pwd)
debug_output=$(PROJDIR)/target/debug
release_output=$(PROJDIR)/target/release

all: debug

debug:
	cargo build
	cd $(PROJDIR)/web && pnpm build 
	mv $(PROJDIR)/web/dist $(debug_output)/web

release:
	cargo build --release
	cd $(PROJDIR)/web && pnpm build
	mv $(PROJDIR)/web/dist $(release_output)/web
