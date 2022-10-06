FROM debian:bookworm-slim

RUN apt-get update \
 && apt-get install -y \
      pandoc \
      wkhtmltopdf \
      texlive \
      build-essential python3-dev python3-pip python3-setuptools python3-wheel python3-cffi libcairo2 libpango-1.0-0 libpangocairo-1.0-0 libgdk-pixbuf2.0-0 libffi-dev shared-mime-info \
 && pip3 install weasyprint \
 && pandoc --version

EXPOSE 8000

RUN useradd -m rocket
USER rocket
WORKDIR /home/rocket
