FROM rust:1.63-bullseye AS backend_build

WORKDIR /app
COPY . /app
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update -y \
  && apt-get install -y --no-install-recommends ca-certificates openssl \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*WORKDIR

COPY --from=backend_build /app/target/release/httpavonz /

CMD ./httpavonz
