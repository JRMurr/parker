FROM rustlang/rust:nightly-slim

WORKDIR /parker
RUN apt-get update && \
    apt-get install -y \
        libssl-dev \
        pkg-config\
        wget \
        && \
    rm -rf /var/lib/apt/lists/* && \
    wget -qO /usr/local/bin/wait-for-it \
        https://raw.githubusercontent.com/vishnubob/wait-for-it/master/wait-for-it.sh \
        && \
    chmod +x /usr/local/bin/wait-for-it && \
    cargo install cargo-watch
