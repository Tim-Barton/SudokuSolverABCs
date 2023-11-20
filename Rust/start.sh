#/bin/sh

set -x

docker build . -t rust

docker run -v $(pwd):$(pwd) -w $(pwd) -it rust
