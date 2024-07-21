test:
	mkdir ./target/debug/src
	cp -a ./src/resources ./target/debug/src/resources/
	cargo build --bin=f_engine --package=f_engine
	
release:
	mkdir ./target/release/src
	cp -a ./src/resources ./target/release/src/resources/
	cargo build --bin=f_engine --package=f_engine --release