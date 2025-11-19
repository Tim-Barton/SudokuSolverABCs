#/bin/sh

set -x

docker build . -t ada

docker run -v $(pwd):$(pwd) -w $(pwd) -it ada 
