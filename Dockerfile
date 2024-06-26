FROM rust:1.70.0-slim-bullseye AS build

# View app name in Cargo.toml
ARG APP_NAME="rust-person-api-example"

WORKDIR /build

COPY Cargo.lock Cargo.toml ./

COPY src src
RUN cargo build --locked --release
RUN cp ./target/release/$APP_NAME /bin/server

FROM debian:bullseye-slim AS final

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    appuser
USER appuser

COPY --from=build /bin/server /bin/
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 3000
CMD ["/bin/server"]
