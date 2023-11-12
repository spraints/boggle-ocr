.PHONY: all
all: target/release/boggle-ocr cached.dict wordle-cheat/config/dictionary

SRCS = $(shell find src -name '*.rs')

target/release/boggle-ocr: $(SRCS) Cargo.toml
	env DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/" \
	  cargo build --release

cached.dict: DICT.json target/release/boggle-ocr
	target/release/boggle-ocr compile -f DICT.json cached.dict

wordle-cheat/config/dictionary: cached.dict
	cp cached.dict wordle-cheat/config/dictionary

DICT.json:
	curl -L -o DICT.js http://insightcoder.com/sw/boggle-dictionary/files/DICT.js
	cat DICT.js | cut -d = -f 2- | sed -e 's/;$$//' > DICT.json
	rm DICT.js
