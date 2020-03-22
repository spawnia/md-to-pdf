FROM rustlang/rust:nightly-buster as builder

WORKDIR /usr/src/md-to-pdf
COPY . .

RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get update \
 && apt-get install -y \
      pandoc \
      wkhtmltopdf \
      weasyprint \
 && pandoc --version

COPY --from=builder /usr/local/cargo/bin/md-to-pdf /usr/bin/md-to-pdf

EXPOSE 8000
CMD ["md-to-pdf"]

RUN useradd -m rocket
USER rocket
WORKDIR /home/rocket
