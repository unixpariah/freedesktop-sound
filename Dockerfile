FROM ubuntu:latest

ENV HOME=/root
ENV PATH="/root/.cargo/bin:$PATH"

RUN apt-get update && \
    apt-get install -y apt-utils curl gcc sound-theme-freedesktop oxygen-sounds deepin-sound-theme && \
    rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf > /tmp/rustup-init.sh && \
    chmod +x /tmp/rustup-init.sh && \
    sh /tmp/rustup-init.sh -y && \
    rm -rf /tmp/rustup-init.sh

RUN cargo --version
RUN rustup install nightly

WORKDIR /app
COPY . /app

CMD ["cargo", "test"]
