FROM ubuntu:18.04

WORKDIR /home/

COPY . .

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:$PATH"
