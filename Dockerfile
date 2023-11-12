FROM rust:1.73.0-buster AS build

WORKDIR /work
ADD Cargo.toml .
ADD Cargo.lock .
RUN mkdir src && echo 'fn main(){}' > src/main.rs && cargo fetch && rm src/main.rs

ADD Makefile Makefile
ADD src      src

RUN make target/release/boggle-ocr cached.dict DICT.json

FROM debian:buster-slim

COPY --from=build /work/target/release/boggle-ocr /usr/bin/boggle-ocr
COPY --from=build /work/cached.dict               /usr/share/boggle-ocr/cached.dict
COPY --from=build /work/DICT.json                 /usr/share/boggle-ocr/DICT.json

ADD assets                                        /usr/share/boggle-ocr/assets

CMD boggle-ocr serve \
  --addr 0.0.0.0:3000 \
  --assets /usr/share/boggle-ocr/assets \
  --dict /usr/share/boggle-ocr/cached.dict \
  --defs /usr/share/boggle-ocr/DICT.json
