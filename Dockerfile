FROM rust:1.45 AS build
# Download the target for static linking.
RUN apt-get update \
  && DEBIAN_FRONTEND=noninteractive apt-get -y install musl-dev musl-tools \
  && rustup target add x86_64-unknown-linux-musl

# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# docker build cache will be used to skip these slow steps.
WORKDIR /usr/src
RUN USER=root cargo new gatodown
WORKDIR /usr/src/gatodown
COPY Cargo.* ./
RUN cargo build --release

# Copy the source and build the application and health checker
COPY src ./src
COPY static ./static
RUN cargo install --target x86_64-unknown-linux-musl --path . \
  && mkdir -p /rootfs/var/lib/www/static /rootfs/bin/ \
  && cp /usr/local/cargo/bin/gatodown /rootfs/bin/ \
  && cp -r /usr/src/gatodown/static /rootfs/var/lib/www/ \
  && chown -R 48.48 /rootfs/var/lib/www/

FROM scratch
COPY --from=build /rootfs/  /
USER 48:48
WORKDIR /var/lib/www/
CMD ["/bin/gatodown"]

