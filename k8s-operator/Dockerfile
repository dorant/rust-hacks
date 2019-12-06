#FROM rust:1.39.0 AS builder
FROM clux/muslrust:1.39.0-stable AS builder
ARG BINARY_NAME

ADD ./target/x86_64-unknown-linux-musl/release/$BINARY_NAME /root/

#FROM scratch
FROM centos:7
RUN curl -LO https://storage.googleapis.com/kubernetes-release/release/v1.16.0/bin/linux/amd64/kubectl \
        && chmod +x ./kubectl \
        && mv ./kubectl /usr/local/bin/kubectl
RUN curl -LO https://get.helm.sh/helm-v3.0.1-linux-amd64.tar.gz \
        && tar -xf helm-v3.0.1-linux-amd64.tar.gz \
        && mv linux-amd64/helm /usr/local/bin/ \
        && rm helm-v3.0.1-linux-amd64.tar.gz

ARG BINARY_NAME
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt \
    SSL_CERT_DIR=/etc/ssl/certs \
    OPERATOR=/usr/local/bin/${BINARY_NAME} \
    USER_UID=1001 \
    USER_NAME=rusty

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /root/$BINARY_NAME ${OPERATOR}

RUN useradd --create-home --shell /bin/bash --uid ${USER_UID} ${USER_NAME}
COPY build/bin /usr/local/bin
RUN  /usr/local/bin/user_setup

ENTRYPOINT ["/usr/local/bin/entrypoint"]

USER ${USER_UID}
