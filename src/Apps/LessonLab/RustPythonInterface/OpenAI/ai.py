import openai
from dotenv import load_dotenv
import os
from pathlib import Path

    
with open("../PythonScrapers/Output.md", "r", encoding="utf-8") as file:
    source_file = file.read()

load_dotenv()

openai.api_key = os.getenv("API_KEY")

prompt = "Can you make a markdown format lesson based on this source: " + source_file

completion = openai.ChatCompletion.create(
  model="gpt-3.5-turbo",
  messages=[
    {"role": "system", "content": "You are a college teacher."},
    {"role": "user", "content": prompt}
  ]
)

#completion['choices'][0]['message']['content'] - the reply of gpt
reply = completion['choices'][0]['message']['content']
print(reply)