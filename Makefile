SRCS = src/wordsearch.rs src/dictionary.rs src/main.rs src/options.rs
target/release/boggle-ocr: $(SRCS) Cargo.toml
	cargo build --release
