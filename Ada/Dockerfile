FROM alpine:edge

RUN echo 'http://dl-cdn.alpinelinux.org/alpine/edge/testing' >> /etc/apk/repositories;\
    apk update

RUN apk add gcc-gnat build-base gprbuild