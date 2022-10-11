FROM lukemathwalker/cargo-chef:0.1.45-rust-1.64-bullseye AS chef

ENV CARGO_HOME=/root/.cargo

RUN mkdir /root/.cargo                                                        &&\
  cd /root/.cargo                                                             &&\
  echo '[source.crates-io]' > config                                          &&\
  echo 'registry = "https://github.com/rust-lang/crates.io-index"' >> config  &&\
  echo 'replace-with = "tencent"' >> config                                   &&\
  echo '[source.tencent]' >> config                                           &&\
  echo 'registry = "http://rust.mirrors.tencent.com/index"' >> config

WORKDIR /app

FROM chef AS planner

COPY hello-world .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS app

COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

COPY hello-world .

RUN cargo build --release

WORKDIR /output

RUN cd /app/target/release/ && cp hello-world hacker /output/

FROM sammyne/gramine:1.2-ubuntu20.04 AS builder

RUN developer_key_path=~/.gramine/developer-key.pem &&\
  mkdir -p $(dirname $developer_key_path)           &&\
  openssl genrsa -3 -out $developer_key_path 3072

WORKDIR /root/gramine/Examples/hello-world

ADD gramine .

COPY --from=app /output/hello-world .

ENV LC_ALL=C.UTF-8 \
    LANG=C.UTF-8

RUN make SGX=1

WORKDIR /output

RUN cp -r /root/gramine/Examples/hello-world .

WORKDIR /output/

COPY --from=app /output/hacker .

FROM sammyne/sgx-dcap:2.17.100.3-dcap1.14.100.3-ubuntu20.04

RUN sed -i 's/archive.ubuntu.com/mirrors.ustc.edu.cn/g' /etc/apt/sources.list &&\
  sed -i 's/security.ubuntu.com/mirrors.ustc.edu.cn/g' /etc/apt/sources.list  &&\
  apt update && apt install -y libprotobuf-c-dev

WORKDIR /gramine

COPY --from=builder /usr/local/bin/gramine-sgx /usr/local/bin/gramine-sgx
COPY --from=builder /usr/local/lib/x86_64-linux-gnu /usr/local/lib/x86_64-linux-gnu

COPY --from=builder /output/* ./

CMD gramine-sgx hello-world 
