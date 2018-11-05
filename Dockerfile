FROM rust:1.30-slim as build

WORKDIR /src
COPY . .

RUN cargo build --release

FROM getynge/upx:3.94-r0 as compress
COPY --from=build /src/target/release/microserve /microserve.big
RUN upx --best --lzma -o /microserve /microserve.big

FROM scratch
COPY --from=compress /microserve /microserve
CMD ["/microserve"]