FROM rustlang/rust:nightly-slim as builder

WORKDIR /usr/src/md-to-pdf
COPY . .

RUN cargo install --path .

FROM debian:sid-slim

RUN apt-get update \
 && apt-get install -y \
      pandoc \
      wkhtmltopdf \
      build-essential python3-dev python3-pip python3-setuptools python3-wheel python3-cffi libcairo2 libpango-1.0-0 libpangocairo-1.0-0 libgdk-pixbuf2.0-0 libffi-dev shared-mime-info \
 && pip3 install weasyprint \
 && pandoc --version

COPY --from=builder /usr/local/cargo/bin/md-to-pdf /usr/bin/md-to-pdf

EXPOSE 8000
CMD ["md-to-pdf"]

RUN useradd -m rocket
USER rocket
WORKDIR /home/rocket

COPY static /home/rocket/static
