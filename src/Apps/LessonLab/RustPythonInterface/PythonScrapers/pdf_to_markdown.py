import sys
import fitz

def pdf_to_markdown():

  fileName = sys.argv[1]

  fileName = "Test.pdf"

  pdf_document = fitz.open(fileName)

  markdown_text = ""

  for page_number in range(pdf_document.page_count):
      page = pdf_document[page_number]
      page_text = page.get_text()
      markdown_text += page_text


pdf_to_markdown()