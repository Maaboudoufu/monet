# --- build ---
FROM rust:1.100-slim AS build
WORKDIR /app
COPY Cargo.toml ./
RUN cargo generate-lock-file
COPY Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked

# --- runtime ---
FROM debian:bookworm-slim
COPY --from=build /app/target/release/pompom /usr/local/bin/pompom
COPY static/img /usr/share/pompom/img
ENV PORT=8080
EXPOSE 8080
ENTRYPOINT ["pompom"]
