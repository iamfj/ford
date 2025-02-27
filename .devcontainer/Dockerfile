FROM node:lts-alpine AS docs
ENV HOST=0.0.0.0
RUN apk add --no-cache curl \
  && npm i docsify-cli -g
EXPOSE 3000
EXPOSE 35729
WORKDIR /home/node
RUN mkdir -p /home/node/docs
CMD ["docsify", "serve", "/home/node/docs", "--port", "3000", "--no-open"]

FROM mcr.microsoft.com/vscode/devcontainers/rust AS devcontainer
RUN curl -1sLf 'https://dl.cloudsmith.io/public/evilmartians/lefthook/setup.deb.sh' | sudo -E bash \
  && sudo apt install lefthook
RUN rustup component add rustfmt
RUN rustup component add clippy
CMD ["sleep", "infinity"]
