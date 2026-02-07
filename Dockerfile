FROM rustlang/rust:nightly-slim as builder

WORKDIR /usr/src/md-to-pdf
COPY . .

RUN cargo install --path .

FROM debian:12-slim

RUN apt-get update \
 && apt-get install --yes \
      pandoc \
      wkhtmltopdf \
      texlive \
      build-essential python3-dev python3-pip python3-setuptools python3-wheel python3-cffi libcairo2 libpango-1.0-0 libpangocairo-1.0-0 libgdk-pixbuf2.0-0 libffi-dev shared-mime-info \
      poppler-utils \
      qpdf \
 && rm -rf /var/lib/apt/lists/* \
 # https://stackoverflow.com/questions/75608323/how-do-i-solve-error-externally-managed-environment-every-time-i-use-pip-3
 && pip3 install --no-cache-dir --break-system-packages weasyprint \
 && pandoc --version

COPY --from=builder /usr/local/cargo/bin/md-to-pdf /usr/local/bin/md-to-pdf

EXPOSE 8000
CMD ["md-to-pdf"]

RUN useradd --create-home rocket
USER rocket
WORKDIR /home/rocket

COPY static /home/rocket/static
COPY Rocket.toml /home/rocket/Rocket.toml
COPY templates /home/rocket/templates 
