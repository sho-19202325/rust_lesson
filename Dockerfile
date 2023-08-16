FROM rust:1.71

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

RUN apt-get update && apt-get install -y bash

CMD ["bash"]