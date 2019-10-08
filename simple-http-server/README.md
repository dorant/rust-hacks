# Simple test of a Rust http-server

## Build Docker image
docker build -t bjornsv/simple-http-server:latest .

## Push image
docker push bjornsv/simple-http-server:latest

## Test Docker image
docker run -d -p 8080:8080 http-server:latest

## Install in K8s
cd charts
[ helm init ]
helm upgrade --install simple-http-server charts/simple-http-server

## Example of modifications when using an Ingress instead of LB
helm upgrade --install \
  --set service.type="NodePort" \
  --set service.nodePort=30600 \
  --set message="Hello service" \
  --set livenessProbe=null \
 simple-http-server charts/simple-http-server
