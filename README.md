<!-- Need to be replaced with banner + logo -->
# LessonLab 
<!-- Need to be replaced with banner + logo -->

**LessonLab** is a cutting-edge AI-based tool designed to revolutionize the creation of educational content, including lessons and quizzes. Powered by the formidable GPT-4, LessonLab offers an unparalleled solution for educators, trainers, and content creators seeking to streamline and enhance the educational content development process.

## Features
- **AI-Powered Content Generation**: Utilizes GPT-4, an advanced AI model, to process text-based files and automatically generate summarized lessons and quizzes.
- **Flexible Output Formats**: Choose from a variety of output formats to suit your needs, such as PDFs, HTML documents, or even interactive online quizzes.
- **User-Friendly Interface**: A straightforward and intuitive interface that makes content generation a breeze.
- **Customizable Summaries**: Tailor the level of detail in your summaries to match your requirements, from concise overviews to comprehensive lesson plans.
- **Automatic Question Generation**: LessonLab can generate quiz questions based on the processed content, saving you time in quiz creation.
- **Local Processing**: Ensure data privacy and security with the option to process files locally, without sending data to external servers.

    ### To be implemented in later iterations
    - **Export and Share**: Easily export your generated lessons and quizzes for distribution or sharing with students or colleagues.
    - **Integration Support**: Seamlessly integrate LessonLab into your existing educational platforms or tools.

# Getting Started

## Prerequisites
- [**Rust**](https://www.rust-lang.org/): This project relies on Rust as its core programming language. You'll need to have Rust installed to build and run the code.

- [**Python**](https://www.python.org/): Python is used to interact with GPT-4's API. Make sure you have Python installed to enable this functionality.

### Libraries used:
- Rust Libraries

    - [Dioxus](https://github.com/DioxusLabs/dioxus) - a Rust library for building apps that run on desktop, web, mobile, and more.
    - [PyO3](https://github.com/PyO3/pyo3) - Rust bindings for Python, including tools for creating native Python extension modules. Running and interacting with Python code from a Rust binary is also supported.

- Python Libraries

    - *To add more later...*

## Configuration

This project uses Rust 1.73.0 and Python 3.12.0 (latest versions as of project initialization). Consult the official documentation for each language to find installation instructions.

Follow the installation instructions for [Dioxus' CLI](https://dioxuslabs.com/learn/0.4/CLI/installation) for support of the project's build commands. 

### Building the project

Development builds will utilize [Dioxus Web](https://dioxuslabs.com/learn/0.4/getting_started/wasm) as it supports Hot Reloading for ease of development.  

>To run Development builds, please refer to the latest documentation for the corresponding command:

```
dx serve --hot-reload
```

Production builds will target Desktop Native ([Dioxus Desktop](https://dioxuslabs.com/learn/0.4/getting_started/desktop)). 

>To run Production builds, please refer to the latest documentation for the corresponding command:

```
dx serve --hot-reload --platform desktop
```
>Note: Dioxus Desktop builds do not actually support Hot Reloading as of 0.4.0

## Licence and Project Status
This project is closed source and not intended for redistribution. The code and associated resources in this repository are provided solely for reference and use within the context of this project.

**Usage Restrictions:**

- You are not permitted to redistribute, copy, or modify this code for external projects or applications without explicit permission.

If you have any questions or need further clarification regarding the use of this code, please contact the project maintainers.