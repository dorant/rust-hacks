# Simple test of a Rust http-server

## Build Docker image
docker build -t http-server:latest .

## Test Docker image
docker run -d -p 8080:8080 http-server:latest
