import pdfplumber
import aspose.words as aw
import openai
import re
from dotenv import load_dotenv
import os

with pdfplumber.open('Proxy Design Pattern Summary.pdf') as pdf:
    # iterate over each page
    for page in pdf.pages:
        # extract text
        text = page.extract_text()
        # print(text)

fileName = "Proxy Design Pattern Summary.pdf"

output = aw.Document()
output.remove_all_children()

input = aw.Document(fileName)
output.append_document(input, aw.ImportFormatMode.KEEP_SOURCE_FORMATTING)

output.save("Output.md")

with open("Output.md", "r", encoding="utf-8") as file:
    markdown_text = file.read()

    markdown_text = re.sub(r'!\[.*\]\(.*\)', '', markdown_text)

    watermark_text_start = r"\*\*Evaluation Only\. Created with Aspose\.Words\. Copyright 2003-2023 Aspose Pty Ltd\.\*\*"
    markdown_text = re.sub(watermark_text_start, '', markdown_text)

    watermark_text_end = r"\*\*Created with an evaluation copy of Aspose\.Words\. To discover the full versions of our APIs please visit: https://products\.aspose\.com/words/\*\*"
    markdown_text = re.sub(watermark_text_end, '', markdown_text)
    #print(markdown_text)

load_dotenv()

openai.api_key = os.getenv("API_KEY")

prompt = "Can you make a markdown format lesson based on this source: " + markdown_text

completion = openai.ChatCompletion.create(
  model="gpt-3.5-turbo",
  messages=[
    {"role": "system", "content": "You are a college teacher."},
    {"role": "user", "content": prompt}
  ]
)

#completion['choices'][0]['message']['content'] - the reply of gpt
print(completion['choices'][0]['message']['content'])