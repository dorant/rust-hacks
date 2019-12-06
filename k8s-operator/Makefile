BINARY_NAME=k8s-operator

.PHONY: all build docker

all: build docker deploy

build:
	docker run -v cargo-cache:/root/.cargo/registry -v "$$PWD:/volume" \
		--rm -it clux/muslrust:1.39.0-stable cargo build --release

docker:
	docker build -t bjornsv/k8s-operator:latest . \
		 --pull --no-cache --build-arg BINARY_NAME=$(BINARY_NAME)

deploy:
	kind load docker-image bjornsv/k8s-operator:latest
	kubectl create  -f manifests/crd.yaml || true
	kubectl delete  -f manifests/operator/deployment.yaml \
			-f manifests/operator/role.yaml \
			-f manifests/operator/role_binding.yaml \
			-f manifests/operator/service_account.yaml \
			-f manifests/cr.yaml || true
	kubectl create  -f manifests/operator/deployment.yaml \
			-f manifests/operator/role.yaml \
			-f manifests/operator/role_binding.yaml \
			-f manifests/operator/service_account.yaml \
			-f manifests/cr.yaml
	kubectl get all --all-namespaces
