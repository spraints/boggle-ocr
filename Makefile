.PHONY: all
all: target/release/boggle-ocr cached.dict

SRCS = src/wordsearch.rs src/dictionary.rs src/main.rs src/options.rs

target/release/boggle-ocr: $(SRCS) Cargo.toml
	cargo build --release

cached.dict: OWL2.json target/release/boggle-ocr
	target/release/boggle-ocr compile -f OWL2.json cached.dict

OWL2.json:
	curl -L -o OWL2.js http://insightcoder.com/sw/boggle-dictionary/files/OWL2.js
	echo 'console.log(JSON.stringify(dictionary));' >> OWL2.js
	node OWL2.js > OWL2.json
	rm OWL2.js
