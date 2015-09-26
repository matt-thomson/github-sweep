FROM quay.io/jimmycuadra/rust:latest

WORKDIR /srv/app

COPY . /srv/app
RUN cargo build --release

CMD ["target/release/github-sweep"]
