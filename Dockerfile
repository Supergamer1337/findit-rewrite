FROM rust:1.93-alpine AS builder

RUN apk add --no-cache musl-dev build-base pkgconfig openssl-dev openssl-libs-static ca-certificates

RUN cargo install dioxus-cli@0.7.1 

# Add the WebAssembly target for the Dioxus frontend
RUN rustup target add wasm32-unknown-unknown

# Copy the Cargo.toml and Cargo.lock files to leverage Docker's caching mechanism for dependencies
WORKDIR /app
COPY Cargo.toml Cargo.lock Dioxus.toml ./

# Build and cache the dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch
RUN cargo build --release
RUN rm src/main.rs

# Copy the actual code files and build the application
COPY src ./src/
COPY assets ./assets/

# Update the file date
RUN touch src/main.rs

# Build using the Dioxus CLI, which will handle both the Rust backend and the WebAssembly frontend
RUN dx build --release

##########################
#    PRODUCTION STAGE    #
##########################
FROM scratch

WORKDIR /app

# Copy server binary from the build stage 
COPY --from=builder /app/target/dx/find-it/release/web/find-it ./find-it
# Copy static files
COPY --from=builder /app/target/dx/find-it/release/web/public ./public
# For some reason images are not in the public folder, so we need to copy them separately
COPY --from=builder /app/assets/images ./public/images/

ENV IP=0.0.0.0
ENV PORT=8080

ENTRYPOINT ["/app/find-it"]
