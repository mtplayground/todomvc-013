FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY target/release/todomvc-leptos /app/todomvc-leptos
COPY target/site /app/target/site
COPY Cargo.toml /app/Cargo.toml
ENV LEPTOS_SITE_ADDR=0.0.0.0:8080
ENV DATABASE_URL=sqlite:/app/data/todos.db?mode=rwc
RUN mkdir -p /app/data
EXPOSE 8080
CMD ["/app/todomvc-leptos"]
