FROM rustlang/rust:nightly-slim AS rust

WORKDIR /usr/src/md-to-pdf
COPY . .

RUN cargo install --path .

FROM debian:trixie-slim

RUN apt-get update \
 && apt-get install --yes --no-install-recommends \
      pandoc \
      texlive \
      python3 \
      python3-pip \
      python3-cffi \
      libcairo2 \
      libpango-1.0-0 \
      libpangocairo-1.0-0 \
      libgdk-pixbuf-2.0-0 \
      libffi-dev \
      shared-mime-info \
      xfonts-base \
      xfonts-75dpi \
      wget \
 && apt-get install --yes --no-install-recommends \
      build-essential \
      python3-dev \
      python3-setuptools \
      python3-wheel \
 && ARCH=$(dpkg --print-architecture) \
 && wget --quiet https://github.com/wkhtmltopdf/packaging/releases/download/0.12.6.1-3/wkhtmltox_0.12.6.1-3.bookworm_${ARCH}.deb \
 && apt-get install --yes --no-install-recommends ./wkhtmltox_0.12.6.1-3.bookworm_${ARCH}.deb \
 && rm wkhtmltox_0.12.6.1-3.bookworm_${ARCH}.deb \
 && pip3 install --no-cache-dir --break-system-packages weasyprint \
 && apt-get purge --yes build-essential python3-dev python3-setuptools python3-wheel wget \
 && apt-get autoremove --yes \
 && rm --recursive --force /var/lib/apt/lists/* \
 && pandoc --version

COPY --from=rust /usr/local/cargo/bin/md-to-pdf /usr/local/bin/md-to-pdf

EXPOSE 8000
CMD ["md-to-pdf"]

RUN useradd --create-home rocket
USER rocket
WORKDIR /home/rocket

COPY static /home/rocket/static
COPY Rocket.toml /home/rocket/Rocket.toml
