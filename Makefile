BIN         ?= /usr/local/bin
TARGETDIR   ?= target/release
CLIENT      ?= "usock"

build:
	@cargo build --release

clean:
	@cargo clean