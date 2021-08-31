# md-to-pdf

A web service for converting markdown to PDF

## Web UI

For quick experimentation, you can use the web version at [https://md-to-pdf.herokuapp.com](https://md-to-pdf.herokuapp.com).
Just paste your markdown and download the converted PDF.

## API

You can convert markdown by sending a `POST` request to `https://md-to-pdf.herokuapp.com`.

    curl --data-urlencode 'markdown=# Heading 1' --output md-to-pdf.pdf https://md-to-pdf.herokuapp.com

| Parameter | Required | Description |
| --- | --- | --- |
| `markdown` | Required | The markdown content to convert |
| `css` | Optional | CSS styles to apply |
| `engine` | Optional |The PDF conversion engine, can be `wkhtmltopdf` or `weasyprint`, defaults to `weasyprint` |

Send data from files like this:

    curl --data-urlencode "markdown=$(cat example.md)" 

## Deploy

A prebuilt container image is available at [Docker Hub](https://hub.docker.com/r/spawnia/md-to-pdf).
You can run it yourself like this:

    docker run -p 8000:8000 spawnia/md-to-pdf

## Built with

- [Rocket - a web framework for Rust](https://rocket.rs)
- [Pandoc - a universal document converter](https://pandoc.org)
- [Codemirror - a text editor for the browser](https://codemirror.net)
