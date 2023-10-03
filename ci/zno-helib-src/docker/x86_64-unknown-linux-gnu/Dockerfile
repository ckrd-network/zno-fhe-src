FROM ubuntu:18.04

RUN apt update && \
    apt dist-upgrade -y && \
    DEBIAN_FRONTEND=noninteractive apt-get update -y && apt-get install -y --no-install-recommends \
      ca-certificates \
      make \
      perl \
      gcc \
      libc6-dev
