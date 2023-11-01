from bs4 import BeautifulSoup
import requests, re
import openai
import os

# paste url here
url = "https://www.javatpoint.com/theory-of-automata"
pageToScrape = requests.get(url)

# Some urls to test:
    # https://sourcemaking.com/design_patterns
    # https://www.w3schools.com/html/
    # https://www.javatpoint.com/theory-of-automata

soup = BeautifulSoup(pageToScrape.text, "html.parser")

# html block elements to read
# paragraphs = soup.findAll('p')
# headers = soup.findAll(re.compile('^h[1-6]$'))

paragraphs = [p.get_text() for p in soup.find_all('p')]
headers = [h.get_text() for h in soup.find_all(re.compile('^h[1-6]$'))]

# Initialize an empty string to store the result
result_string = ""

# Iterate through headers and paragraphs and concatenate them
for header, paragraph in zip(headers, paragraphs):
    result_string += f"{header}\n{paragraph}\n\n"

# Print the result
print(result_string)

def scrapeWebText():

    soup = BeautifulSoup(pageToScrape.text, "html.parser")

    # html block elements to read
    paragraphs = soup.findAll('p')
    headers = soup.findAll(re.compile('^h[1-6]$'))

    for header, paragraph in zip(headers, paragraphs):
        print(header.text + " \n\n " + paragraph.text)
    
    # for header in headers:
    #     print(header.text)

def scrapeWebHtml():

    # html and css of the website
    print(pageToScrape.text)


#scrapeWebText()
# scrapeWebHtml()


#GPT
# openai.api_key = os.getenv.API_KEY


# prompt = "Can you explain this: " + topic


# completion = openai.ChatCompletion.create(
#   model="gpt-3.5-turbo",
#   messages=[
#     {"role": "system", "content": "You are a college teacher."},
#     {"role": "user", "content": prompt}
#   ]
# )

# #completion['choices'][0]['message']['content'] - the reply of gpt
# print(completion['choices'][0]['message']['content'])