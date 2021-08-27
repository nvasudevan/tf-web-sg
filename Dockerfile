FROM rust:1.54-slim

ARG tf_web_bg_ver
ENV TF_WEB_BG_VER=$tf_web_bg_ver

WORKDIR /var/tmp
RUN apt-get update -y && apt install -y wget unzip curl
RUN wget https://github.com/nvasudevan/tf-web-sg/releases/download/${TF_WEB_BG_VER}/tf-web-sg_${TF_WEB_BG_VER}_x86_64-unknown-linux-musl.zip
RUN unzip tf-web-sg_${TF_WEB_BG_VER}_x86_64-unknown-linux-musl.zip && mv ./tf-web-bg /opt
EXPOSE 8080

CMD ["/opt/tf-web-bg"]
