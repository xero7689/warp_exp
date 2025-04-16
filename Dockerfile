FROM rust:1.85.0 as builder

WORKDIR /usr/src/app

COPY . .

RUN cargo install --path .


# Create a smaller runtime image
FROM debian:bookworm-slim

#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/local/cargo/bin/warp_exp /app/warp_exp
COPY log4rs.yaml /app/log4rs.yaml

EXPOSE 3030

# Command to run the application
CMD ["/app/warp_exp"]
