use pyo3::{
    prelude::*,
    types::{IntoPyDict, PyModule},
};


pub fn scrape_pdf() -> PyResult<String> {
    pyo3::prepare_freethreaded_python();
Python::with_gil(|py| {
    let scraper = PyModule::from_code(
        py,
        r#"
import fitz

def scrape(src):
    fileName = src
    pdf_document = fitz.open(fileName)
    markdown_text = ""
    for page_number in range(pdf_document.page_count):
        page = pdf_document[page_number]
        page_text = page.get_text()
        markdown_text += page_text
    return markdown_text
    "#,
        "scraper.py",
        "scraper",
    )?;

    let lesson_source: String = scraper.getattr("scrape")?.call(("../test.pdf",), None)?.extract()?;

    Ok(lesson_source)
})
}