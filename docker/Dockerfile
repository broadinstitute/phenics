FROM ubuntu:22.04 AS builder

WORKDIR build_phenics
RUN apt-get -y update && \
    apt-get -y upgrade && \
    apt-get -y install curl git build-essential pkg-config openssl libssl-dev && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain 1.61.0 -y && \
    export PATH=$PATH:$HOME/.cargo/bin && \
    bash $HOME/.cargo/env
RUN mkdir git && \
    cd git && \
    git clone https://github.com/broadinstitute/phenics.git && \
    cd phenics && \
    git checkout v0.2.27  && \
    export PATH=$PATH:$HOME/.cargo/bin && \
    cargo install --path . --root /usr/local && \
    cd ../..  && \
    rm -r git

FROM ubuntu:22.04

RUN apt-get -y update && \
    apt-get -y upgrade && \
    apt-get -y install openssl ca-certificates && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/bin/phenics /usr/local/bin/phenics
