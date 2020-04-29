FROM debian:bullseye-slim
WORKDIR /app
ADD target/release/stream8 .
CMD ["/app/stream8"]