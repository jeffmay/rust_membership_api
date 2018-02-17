#!/bin/bash

# We need to use a script to encapsulate our build here
docker run -v $(pwd):/home/rust/src ekidd/rust-musl-builder bash .containerbuild.sh

