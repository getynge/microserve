FROM rust:1.30 as build

WORKDIR src
COPY . .

RUN cargo build --release
RUN strip target/release/microserve

FROM getynge/upx:3.94-r0 as compress
COPY --from=build src/target/release/microserve release/microserve
RUN upx --best --lzma -o microserve release/microserve

FROM scratch
COPY --from=build /lib/x86_64-linux-gnu/ld-2.24.so /lib64/ld-linux-x86-64.so.2
COPY --from=build /lib/x86_64-linux-gnu/libdl.so.2 \
 /lib/x86_64-linux-gnu/librt.so.1 \
 /lib/x86_64-linux-gnu/libpthread.so.0 \
 /lib/x86_64-linux-gnu/libgcc_s.so.1 \
 /lib/x86_64-linux-gnu/libc.so.6 \
 /lib/x86_64-linux-gnu/
COPY --from=compress /microserve /microserve
CMD ["/microserve"]