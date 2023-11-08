use pyo3::{
    prelude::*,
    types::{IntoPyDict, PyModule},
    exceptions,
};

//TODO:change this accordingly
use crate::scrape_pdf;

pub fn generate() -> PyResult<String> {
Python::with_gil(|py| {
    pyo3::prepare_freethreaded_python();

    let lesson_source = match scrape_pdf() {
        Ok(source) => source,
        Err(_) => return Err(PyErr::new::<exceptions::PyException, _>("Failed to scrape PDF")),
    };

    let lesson_generator = PyModule::from_code(
        py,
        r#"
from dotenv import load_dotenv
import os
import openai
def generate_lesson(source):
    load_dotenv()

    openai.api_key = os.getenv("API_KEY")

    prompt = "Can you make a markdown format lesson based on this source: " + source

    completion = openai.ChatCompletion.create(
    model="gpt-3.5-turbo",
    messages=[
        {"role": "system", "content": "You are a college teacher."},
        {"role": "user", "content": prompt}
        ]
    )
    reply = completion['choices'][0]['message']['content']
    return reply
    "#,
        "lesson_generator.py",
        "lesson_generator",
    )?;

    let generated_lesson:String = lesson_generator
    .getattr("generate_lesson")?
    .call((lesson_source,), None)?
    .extract()?;

    Ok(generated_lesson)
})
}



