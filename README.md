# md-to-pdf

A web service for converting Markdown to PDF

## Web UI

For quick experimentation, you can use [the web version](https://md-to-pdf.fly.dev) hosted on [Fly.io](https://fly.io).
Paste your Markdown and download the converted PDF.

Availability of the service is not guaranteed, see [Fly.io status](https://status.flyio.net) when it is down.
If you need guaranteed availability, [deploy it yourself](#deploy).

## API

You can convert Markdown by sending a `POST` request to `https://md-to-pdf.fly.dev`.

```shell
curl --data-urlencode 'markdown=# Heading 1' --output md-to-pdf.pdf https://md-to-pdf.fly.dev
```

| Parameter  | Required | Description                                                                                           |
|------------|----------|-------------------------------------------------------------------------------------------------------|
| `markdown` | Required | The markdown content to convert                                                                       |
| `css`      | Optional | CSS styles to apply                                                                                   |
| `engine`   | Optional | The PDF conversion engine, can be `weasyprint`, `wkhtmltopdf` or `pdflatex`, defaults to `weasyprint` |

Send data from files like this:

```shell
curl --data-urlencode "markdown=$(cat example.md)" 
```

## Deploy

A prebuilt container image is available at [Docker Hub](https://hub.docker.com/r/spawnia/md-to-pdf).
The container starts up the web service and listens for HTTP on port 8000.

You can run it yourself like this:

```shell
docker run --publish=8000:8000 spawnia/md-to-pdf
```

You may configure the webserver through [Rocket environment variables](https://rocket.rs/guide/v0.5/configuration#environment-variables).
For example, you could allow larger payloads by increasing the limit for form data:

```dotenv
ROCKET_LIMITS={form="1MiB"}
```

## Built with

- [Rocket - a web framework for Rust](https://rocket.rs)
- [Pandoc - a universal document converter](https://pandoc.org)
- [Codemirror - a text editor for the browser](https://codemirror.net)
