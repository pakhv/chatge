FROM rust:latest AS build

WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
ENV CHATGE_OLLAMA_HOST=host.docker.internal:11434
ENV CHATGE_OLLAMA_MODEL=llama2

WORKDIR /app
COPY --from=build /app/target/release/chatge /app
COPY --from=build /app/static /app/static

CMD [ "./chatge" ]