# md-to-pdf

**A modern web service for converting Markdown to PDF with customizable templates and improved fonts.**  
This project introduces a user-friendly way to convert Markdown to PDF with advanced features such as the use of templates for headers and footers, which can be customized via API calls. The output is further enhanced with better typography for a polished look.

## Key Features
- **Customizable Templates:** Place `.html` files in the `templates` folder and dynamically assign them to headers or footers.
- **Improved Fonts:** Updated fonts for better readability and a professional PDF output.
- **Easy-to-Use Web UI:** Convert Markdown to PDF through a simple web interface.
- **API Integration:** Programmatically convert Markdown to PDF with support for custom CSS and various conversion engines.

---

## Web UI

Use the web UI locally. See the [Deploy](#deploy) section for more information.

---

## API

Convert Markdown programmatically with a simple `POST` request to the API endpoint:

```bash
curl --data-urlencode 'markdown=# Heading 1' --output md-to-pdf.pdf https://md-to-pdf.fly.dev
```

### Parameters

| Parameter          | Required | Description                                                                                             |
|--------------------|----------|---------------------------------------------------------------------------------------------------------|
| `markdown`         | ✔️       | The Markdown content to convert                                                                         |
| `css`              | ❌       | Optional CSS styles to apply                                                                            |
| `engine`           | ❌       | Choose from `weasyprint`, `wkhtmltopdf`, or `pdflatex`. Defaults to `weasyprint`                        |
| `header_template`  | ❌       | Specify a custom header template from the `templates` folder                                             |
| `footer_template`  | ❌       | Specify a custom footer template from the `templates` folder                                             |

To send data from a file:

```bash
curl --data-urlencode "markdown=$(cat example.md)" --output result.pdf https://md-to-pdf.fly.dev
```

---

## Deploy

Run your own instance of the service by using the pre-built container image available on [Docker Hub](https://hub.docker.com/r/spawnia/md-to-pdf):

```bash
docker run --publish=8000:8000 spawnia/md-to-pdf
```

To start the project locally with Docker:

```bash
make serve
```

This builds the Docker image and starts the Rust-based API server.

---

## Built With
- [**Rocket**](https://rocket.rs) - A web framework for Rust
- [**Pandoc**](https://pandoc.org) - A universal document converter
- [**Codemirror**](https://codemirror.net) - A text editor for the browser

---

## Contributing

Contributions are welcome! Fork the repo and submit a pull request. For significant changes, please open an issue first to discuss your ideas.

---

## License & Acknowledgements

This project is a fork of the original `md-to-pdf` created by [Spawnia](https://github.com/Spawnia/md-to-pdf). You can find the original version [here](https://github.com/Spawnia/md-to-pdf). Thank you to the original contributors for laying the groundwork for this project.