FROM ragnaroek/rust-raspberry:1.41.1

RUN apt-get update && \
  apt-get install -y libssl-dev


