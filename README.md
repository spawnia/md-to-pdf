# md-to-pdf

A web service for converting markdown to PDF

## Usage

For quick experimentation, you can use the web version at [https://md-to-pdf.herokuapp.com/](https://md-to-pdf.herokuapp.com/).
Just paste your markdown and download the converted PDF.

## API

You can convert markdown by sending a `POST` request to `https://md-to-pdf.herokuapp.com/`.
Send a form parameter `markdown` with the content to convert:

    curl -X POST -d 'markdown=# Heading 1' https://md-to-pdf.herokuapp.com/

You can also style the markdown through CSS:

    curl -X POST -d 'markdown=# Heading 1' -d 'css=h1 { color: red; }' https://md-to-pdf.herokuapp.com/

Depending on what features you prefer and the output that works best, you can
choose between two pdf conversion engines: `wkhtmltopdf` and `weasyprint`:

    curl -X POST -d 'markdown=# Heading 1' -d 'engine=weasyprint' https://md-to-pdf.herokuapp.com/

## Built with

- [Rocket - a web framework for Rust](https://rocket.rs/)
- [Codemirror - a text editor for the browser](https://codemirror.net/)
- [Pandoc - a universal document converter](https://pandoc.org/)
