FROM phusion/baseimage

ENV RUSTUP_HOME=/rust
ENV CARGO_HOME=/cargo
ENV PATH=/cargo/bin:/rust/bin:$PATH

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y

RUN rustup default nightly-2017-05-02

RUN apt-get update && \
      apt-get -y install sudo gcc

RUN useradd -m docker && echo "docker:docker" | chpasswd && adduser docker sudo

ENV APP_ROOT=/var/www/demo
RUN mkdir -p $APP_ROOT

WORKDIR $APP_ROOT
COPY Cargo.toml Cargo.lock $APP_ROOT/
COPY src $APP_ROOT/src
RUN cargo build --release

EXPOSE 80
CMD sudo env "PATH=$PATH" ROCKET_ENV=production cargo run --release
