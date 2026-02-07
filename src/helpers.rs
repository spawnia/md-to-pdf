use crate::types::*;
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempfile::Builder;

/// The censor replacement HTML block used to replace CENSOR tags in markdown
const CENSOR_REPLACEMENT: &str = "<div style=\"width:100% !important; max-width:100% !important; margin-top:0 !important; margin-bottom:0 !important; margin-left:0 !important; margin-right:0 !important; padding:0 !important; overflow:hidden !important; position:relative !important; left:0 !important; right:auto !important; box-sizing:border-box !important; text-align:left !important;\"><img src=\"static/blured.png\" alt=\"CONTENU PREMIUM - Achetez le rapport complet\" style=\"width:100% !important; height:auto !important; display:block !important; margin:0 !important; padding:0 !important; float:none !important;\"></div>";

/// Process CENSOR tags in markdown, replacing them with the blurred image block
pub fn process_censor(markdown: &str) -> String {
    let mut result = markdown.to_string();
    result = result.replace("{{CENSOR}}", CENSOR_REPLACEMENT);
    result = result.replace("<CENSOR>", CENSOR_REPLACEMENT);
    result = result.replace("{{ CENSOR }}", CENSOR_REPLACEMENT);
    result = result.replace("{{CENSOR }}", CENSOR_REPLACEMENT);
    result = result.replace("{{ CENSOR}}", CENSOR_REPLACEMENT);
    result
}

/// Build a combined CSS file from default.css + custom CSS + options-generated CSS
pub fn build_css(custom_css: Option<&str>, options: Option<&PdfOptions>) -> Result<tempfile::TempPath, AppError> {
    let default_css = fs::read_to_string("templates/default.css")
        .map_err(|e| AppError::Io(e))?;

    let options_css = options.map(|opts| options_to_css(opts)).unwrap_or_default();

    let css_content = match custom_css {
        Some(css) => format!("{}\n{}\n{}", default_css, options_css, css),
        None => format!("{}\n{}", default_css, options_css),
    };

    let mut css_file = Builder::new().suffix(".css").tempfile()?;
    css_file.write_all(css_content.as_bytes())?;
    Ok(css_file.into_temp_path())
}

/// Convert PdfOptions into CSS @page rules
pub fn options_to_css(opts: &PdfOptions) -> String {
    let mut rules = Vec::new();

    if let Some(ref size) = opts.paper_size {
        let size_str = size.to_string();
        let orientation_str = match opts.orientation {
            Some(Orientation::Landscape) => " landscape",
            _ => "",
        };
        rules.push(format!("size: {}{};", size_str, orientation_str));
    } else if let Some(Orientation::Landscape) = opts.orientation {
        rules.push("size: A4 landscape;".to_string());
    }

    if let Some(ref margins) = opts.margins {
        let top = margins.top.as_deref().unwrap_or("2cm");
        let right = margins.right.as_deref().unwrap_or("2cm");
        let bottom = margins.bottom.as_deref().unwrap_or("2cm");
        let left = margins.left.as_deref().unwrap_or("2cm");
        rules.push(format!("margin: {} {} {} {};", top, right, bottom, left));
    }

    if opts.page_numbers.unwrap_or(false) {
        let format = opts
            .page_number_format
            .as_deref()
            .unwrap_or("counter(page)");
        rules.push(format!(
            "@bottom-center {{ content: {}; font-size: 10pt; color: #666; }}",
            format
        ));
    }

    if rules.is_empty() {
        return String::new();
    }

    // Build the @page block; the @bottom-center must be nested inside @page
    let mut page_rules = Vec::new();
    let mut nested_rules = Vec::new();

    for rule in &rules {
        if rule.starts_with('@') {
            nested_rules.push(rule.as_str());
        } else {
            page_rules.push(rule.as_str());
        }
    }

    let mut css = String::from("@page {\n");
    for r in &page_rules {
        css.push_str(&format!("  {}\n", r));
    }
    for r in &nested_rules {
        css.push_str(&format!("  {}\n", r));
    }
    css.push_str("}\n");

    // Watermark via body::after
    if let Some(ref watermark) = opts.watermark {
        css.push_str(&format!(
            r#"body::after {{
  content: "{}";
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%) rotate(-45deg);
  font-size: 80pt;
  color: rgba(0, 0, 0, 0.06);
  z-index: 9999;
  pointer-events: none;
  white-space: nowrap;
}}
"#,
            watermark
        ));
    }

    css
}

/// Resolve header/footer: inline HTML takes priority over template file
pub fn resolve_header_footer(
    inline_html: Option<&str>,
    template_name: Option<&str>,
) -> Result<Option<tempfile::TempPath>, AppError> {
    // Inline HTML takes priority
    if let Some(html) = inline_html {
        if !html.is_empty() {
            let mut file = Builder::new().suffix(".html").tempfile()?;
            file.write_all(html.as_bytes())?;
            return Ok(Some(file.into_temp_path()));
        }
    }

    // Fall back to template file
    if let Some(name) = template_name {
        if !name.is_empty() {
            let current_dir = env::current_dir()?;
            let path = current_dir.join("templates").join(name);
            if path.exists() {
                let content = fs::read_to_string(&path)?;
                let mut file = Builder::new().suffix(".html").tempfile()?;
                file.write_all(content.as_bytes())?;
                return Ok(Some(file.into_temp_path()));
            } else {
                return Err(AppError::NotFound(format!(
                    "Template file not found: {}",
                    name
                )));
            }
        }
    }

    Ok(None)
}

/// Run pandoc to convert markdown to PDF
pub fn run_pandoc(
    markdown: &str,
    css_path: &str,
    engine: &PdfEngine,
    options: Option<&PdfOptions>,
    header_path: Option<&str>,
    footer_path: Option<&str>,
) -> Result<tempfile::TempPath, AppError> {
    let pdf_temp = Builder::new().suffix(".pdf").tempfile()?;
    let pdf_path = pdf_temp.path().to_str().expect("Non UTF-8 path");

    let mut cmd = Command::new("pandoc");
    cmd.arg("--from=markdown+raw_html")
        .arg("--standalone")
        .arg("--to=html5")
        .arg("--variable=geometry:margin=1.5cm")
        .arg("--variable=papersize=a4")
        .arg(format!("--output={}", pdf_path))
        .arg(format!("--pdf-engine={}", engine))
        .arg(format!("--css={}", css_path));

    // TOC support
    if let Some(opts) = options {
        if opts.toc.unwrap_or(false) {
            cmd.arg("--toc");
            if let Some(depth) = opts.toc_depth {
                cmd.arg(format!("--toc-depth={}", depth));
            }
        }
    }

    if let Some(header) = header_path {
        cmd.arg(format!("--include-in-header={}", header));
    }
    if let Some(footer) = footer_path {
        cmd.arg(format!("--include-after-body={}", footer));
    }

    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut process = cmd.spawn()?;
    process
        .stdin
        .as_mut()
        .unwrap()
        .write_all(markdown.as_bytes())?;
    let output = process.wait_with_output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        error!("Pandoc failed: {}", stderr);
        return Err(AppError::ProcessFailed {
            message: "Pandoc conversion failed".to_string(),
            stderr,
        });
    }

    Ok(pdf_temp.into_temp_path())
}

/// Run weasyprint to convert HTML to PDF directly (no pandoc)
pub fn run_weasyprint(html: &str, css_path: &str) -> Result<tempfile::TempPath, AppError> {
    // Write HTML to temp file
    let mut html_file = Builder::new().suffix(".html").tempfile()?;
    html_file.write_all(html.as_bytes())?;
    let html_path = html_file.into_temp_path();
    let html_path_str = html_path.to_str().expect("Non UTF-8 path");

    let pdf_temp = Builder::new().suffix(".pdf").tempfile()?;
    let pdf_path = pdf_temp.path().to_str().expect("Non UTF-8 path").to_string();

    let output = Command::new("weasyprint")
        .arg(html_path_str)
        .arg(&pdf_path)
        .arg("--stylesheet")
        .arg(css_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        error!("Weasyprint failed: {}", stderr);
        return Err(AppError::ProcessFailed {
            message: "Weasyprint conversion failed".to_string(),
            stderr,
        });
    }

    Ok(pdf_temp.into_temp_path())
}

/// Save the PDF to public/ dir and return the download URL, or return the file path for direct response
pub fn save_pdf(
    pdf_path: &Path,
    client_id: &str,
    pdf_name: &str,
) -> Result<String, AppError> {
    let client_dir = Path::new("public").join("pdf").join(client_id);
    fs::create_dir_all(&client_dir)?;

    let final_pdf_name = if pdf_name.ends_with(".pdf") {
        pdf_name.to_string()
    } else {
        format!("{}.pdf", pdf_name)
    };

    let out_path = client_dir.join(&final_pdf_name);
    fs::copy(pdf_path, &out_path)?;

    Ok(format!("/download/{}/{}", client_id, final_pdf_name))
}

/// Convert the first page of a PDF to a PNG using pdftoppm
pub fn pdf_to_png(pdf_path: &Path) -> Result<Vec<u8>, AppError> {
    let png_prefix = Builder::new().prefix("preview-").tempfile()?;
    let prefix_path = png_prefix.path().to_str().expect("Non UTF-8 path").to_string();

    let output = Command::new("pdftoppm")
        .arg("-png")
        .arg("-f")
        .arg("1")
        .arg("-l")
        .arg("1")
        .arg("-r")
        .arg("150")
        .arg(pdf_path.to_str().expect("Non UTF-8 path"))
        .arg(&prefix_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(AppError::ProcessFailed {
            message: "pdftoppm failed".to_string(),
            stderr,
        });
    }

    // pdftoppm creates files like prefix-1.png
    let png_path = format!("{}-1.png", prefix_path);
    // Some versions may use prefix-01.png
    let png_path2 = format!("{}-01.png", prefix_path);

    let actual_path = if Path::new(&png_path).exists() {
        png_path
    } else if Path::new(&png_path2).exists() {
        png_path2
    } else {
        return Err(AppError::ProcessFailed {
            message: "PNG file not found after pdftoppm".to_string(),
            stderr: String::new(),
        });
    };

    let png_data = fs::read(&actual_path)?;
    let _ = fs::remove_file(&actual_path);
    Ok(png_data)
}

/// Resolve a /download/... path to the actual filesystem path with validation
pub fn resolve_pdf_path(url: &str) -> Result<PathBuf, AppError> {
    // Accept paths like /download/client_id/file.pdf
    let stripped = url.trim_start_matches('/');
    let stripped = stripped.strip_prefix("download/").unwrap_or(stripped);

    let path = Path::new("public").join("pdf").join(stripped);

    // Validate: prevent path traversal
    let canonical = path.canonicalize().map_err(|_| {
        AppError::NotFound(format!("PDF not found: {}", url))
    })?;

    let base = Path::new("public")
        .join("pdf")
        .canonicalize()
        .map_err(|_| AppError::NotFound("PDF directory not found".to_string()))?;

    if !canonical.starts_with(&base) {
        return Err(AppError::BadRequest("Invalid PDF path".to_string()));
    }

    if !canonical.exists() {
        return Err(AppError::NotFound(format!("PDF not found: {}", url)));
    }

    Ok(canonical)
}
