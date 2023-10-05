#!/usr/bin/env bash
path="$(pwd)"
podman run --rm \
--privileged \
--userns="" \
-v $path/zno-helib-src/helib:/code \
-v $path/zno-helib-src/scripts:/code/scripts \
-v $path/zno-helib-src/scripts/Doxyfile:/Doxyfile \
docker.io/greenbone/doxygen doxygen /Doxyfile
