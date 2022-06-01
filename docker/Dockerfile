FROM rust:1.60 AS builder

WORKDIR build_phenics
RUN mkdir git && \
    cd git && \
    git clone https://github.com/broadinstitute/phenics.git && \
    cd phenics && \
    git checkout v0.2.0  && \
    cargo install --path . --root /usr/local && \
    cd ../..  && \
    rm -r git

FROM ubuntu:22.04

RUN apt-get -y update && \
    apt-get -y upgrade && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/bin/phenics /usr/local/bin/phenics
