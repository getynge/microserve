FROM getynge/rust:1.30.0-alpine as build

WORKDIR src
COPY Cargo.toml Cargo.lock ./
COPY src src

RUN cargo build --release
RUN strip target/release/microserve

FROM gruebel/upx:edge as compress
COPY --from=build src/target/release/microserve release/microserve
RUN upx --best --lzma --exact -o microserve release/microserve

FROM scratch
COPY --from=build /lib/ld-musl-x86_64.so.1 /lib/ld-musl-x86_64.so.1
COPY --from=build /usr/lib/libgcc_s.so.1 /usr/lib/libgcc_s.so.1
COPY --from=compress /microserve /microserve