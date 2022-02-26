SRCS = src/wordsearch.rs src/dictionary.rs src/main.rs src/options.rs
target/release/boggle-ocr: $(SRCS) Cargo.toml
	cargo build --release

OWL2.json:
	curl -L -o OWL2.js http://insightcoder.com/sw/boggle-dictionary/files/OWL2.js
	echo 'console.log(JSON.stringify(dictionary));' >> OWL2.js
	node OWL2.js > OWL2.json
	rm OWL2.js
