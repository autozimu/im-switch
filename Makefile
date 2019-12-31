all:
	cargo build --release
	ln -s ${PWD}/target/release/im-switch /usr/local/bin/
