# syntax=docker/dockerfile:1
FROM rust:1.75-slim as build
WORKDIR /
COPY . .
#RUN apt-get update
#RUN apt-get install -y pkg-config curl
#RUN apt-get install -y libssl-dev openssl
RUN ["cargo", "build", "--release"]

FROM python:3.12-slim 
COPY --from=build /target/release/sherlock-api /sherlock-api
COPY --from=build /sherlock /sherlock
RUN pip install -r "/sherlock/sherlock/requirements.txt"
VOLUME /data
ARG PORT
CMD ["/sherlock-api"]
EXPOSE $PORT