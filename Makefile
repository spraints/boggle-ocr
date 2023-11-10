.PHONY: all
all: target/release/boggle-ocr cached.dict wordle-cheat/config/dictionary

# find src -name '*.rs' -exec echo 'SRCS =' {} +
SRCS = src/wordle.rs src/skew.rs src/options.rs src/server.rs src/wordsearch.rs src/main.rs src/dictionary.rs

target/release/boggle-ocr: $(SRCS) Cargo.toml
	env DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/" \
	  cargo build --release

cached.dict: DICT.json target/release/boggle-ocr
	target/release/boggle-ocr compile -f DICT.json cached.dict

wordle-cheat/config/dictionary: cached.dict
	cp cached.dict wordle-cheat/config/dictionary

DICT.json:
	curl -L -o DICT.js http://insightcoder.com/sw/boggle-dictionary/files/DICT.js
	echo ';console.log(JSON.stringify(dictionary));' >> DICT.js
	node DICT.js > DICT.json
	rm DICT.js
