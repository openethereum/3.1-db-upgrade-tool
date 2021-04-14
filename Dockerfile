FROM debian:stable as build
ARG CARGO_TARGET_DIR=/tmp
RUN apt-get update && \
  apt-get -y install cargo clang llvm
ADD src src
ADD Cargo.* .
ENV RUSTFLAGS=" -Clink-arg=-s -Ctarget-feature=+aes,+sse2,+ssse3"
RUN cargo build --verbose --release

FROM debian:stable-slim as runtime
ARG CARGO_TARGET_DIR=/tmp
COPY --from=build $CARGO_TARGET_DIR/release/oe-upgrade-db-3-1 /usr/local/bin
