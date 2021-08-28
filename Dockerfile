FROM rust:1.54-slim

WORKDIR /var/tmp
COPY . tf-web-bg
RUN cd tf-web-bg && cargo build --release && cp ./target/release/tf-web-bg /opt/tf-web-bg
RUN rm -rf tf-web-bg
EXPOSE 8080

CMD ["/opt/tf-web-bg"]
