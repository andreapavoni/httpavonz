###################### BACKEND BUILD ###########################################
FROM rust:1.63-bullseye AS backend_build

WORKDIR /app
COPY . /app
RUN cargo build --release

###################### RUNTIME IMAGE ###########################################
FROM debian:bullseye-slim

RUN apt-get update -y \
  && apt-get install -y --no-install-recommends ca-certificates openssl \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*WORKDIR

# Run as "app" user
RUN useradd -d /app -ms /bin/bash app

USER app
WORKDIR /app

COPY --from=backend_build /app/target/release/httpavonz /app/httpavonz

CMD ["/app/httpavonz"]
