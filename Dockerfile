FROM phusion/baseimage

RUN curl -s https://static.rust-lang.org/rustup.sh | sh -s -- \
  --channel=nightly --prefix=/usr --disable-sudo
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
