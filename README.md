# 🚀 md-to-pdf by AI SmartTalk

**A powerful fork of the original md-to-pdf with advanced features for professional document generation.**

This enhanced version, maintained by [AI SmartTalk](https://aismarttalk.tech), extends the original markdown-to-PDF converter with sophisticated features including customizable headers/footers and intelligent content censoring capabilities.

## ✨ Enhanced Features

- **🔒 Document Censoring:** Selectively blur confidential sections while maintaining document layout
- **📄 Customizable Headers & Footers:** Apply professional templates to your documents 
- **🎨 Improved Styling:** Enhanced CSS for better typography and visual presentation
- **🔄 Multiple PDF Engines:** Support for Weasyprint, Wkhtmltopdf, and Pdflatex

## 🔍 How to Use Document Censoring

Easily hide sensitive information in your PDFs with our censoring feature:

```markdown
# Document with Confidential Information

## Public Section
This content will remain visible to everyone.

## Restricted Information
{{CENSOR}}

## Conclusion
Back to publicly visible content.
```

The `{{CENSOR}}` tag can be used in any of these formats:
- `{{CENSOR}}`
- `<CENSOR>`
- `{{ CENSOR }}`
- `{{CENSOR }}`
- `{{ CENSOR}}`

When converted, censored sections appear as blurred areas, clearly indicating restricted content while preserving document flow.

## 🏷️ Header & Footer Templates

Create professional documents with custom headers and footers:

1. Place your custom template files in the `templates` folder with `.html` extension
2. Reference them in your API calls with `header_template` and `footer_template` parameters

Example templates are included to get you started!

## 🔌 API Usage

Convert Markdown with a simple POST request:

```bash
curl --data-urlencode 'markdown=# Heading 1' \
     --data-urlencode 'header_template=corporate_header.html' \
     --data-urlencode 'footer_template=page_numbers.html' \
     --output document.pdf \
     https://your-deployment-url
```

### Parameters

| Parameter          | Required | Description                                                           |
|--------------------|----------|-----------------------------------------------------------------------|
| `markdown`         | ✅       | The Markdown content to convert                                       |
| `css`              | ❌       | Optional CSS styles to apply                                          |
| `engine`           | ❌       | Choose from `weasyprint`, `wkhtmltopdf`, or `pdflatex`               |
| `header_template`  | ❌       | Specify a custom header template from the `templates` folder          |
| `footer_template`  | ❌       | Specify a custom footer template from the `templates` folder          |
| `client_id`        | ❌       | Optional client identifier for document tracking                      |
| `pdf_name`         | ❌       | Custom name for the generated PDF                                     |
| `blurred_paragraphs`| ❌      | Alternative way to specify paragraphs to blur (array of indexes)      |

## 🔧 Deployment

Run your own instance using Docker:

```bash
docker run --publish=8000:8000 aismarttalk/md-to-pdf
```

For local development:

```bash
make serve
```

## 🌐 Web Interface

A user-friendly web interface is available for quick conversions at `http://localhost:8000` when running locally.

## 🔄 Compatibility

- Works across all major browsers and operating systems
- PDFs can be viewed in any standard PDF reader
- API can be integrated with any system that supports HTTP requests

## 🙏 Acknowledgements

This project is a fork of the original `md-to-pdf` created by [Spawnia](https://github.com/Spawnia/md-to-pdf). We extend our gratitude to the original contributors while adding significant new functionality.

## 📝 License

This project maintains the same license as the original repository.

---

Developed with ❤️ by [AI SmartTalk](https://aismarttalk.tech)