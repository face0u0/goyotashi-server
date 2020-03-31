FROM rust:latest

RUN apt-get update -qq && \
    apt-get install -y build-essential \
                       libpq-dev

RUN mkdir /goyotashi-server
ENV GOYOTASHI_ROOT /goyotashi-server
WORKDIR $GOYOTASHI_ROOT

ADD . $GOYOTASHI_ROOT
RUN cargo build

EXPOSE 8000


