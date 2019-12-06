# Test of building a Rust k8s-operator

## Build Docker image
```
make
```

## Push image
```
docker push bjornsv/k8s-operator:latest
```


## Upload image to K8s (kind)
```
kind load docker-image bjornsv/k8s-operator:latest
```

## Install

Install CRD
```
kubectl create -f manifests/crd.yaml
kubectl get crd
```

Install operator
```
kubectl create -f manifests/operator
```


## Remove
kubectl delete -f manifests/operator
kubectl delete -f manifests/crd.yaml
