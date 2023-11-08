from bs4 import BeautifulSoup
import requests, re
import openai
import os
import friendly_check as check

# paste url here
url = "https://www.javatpoint.com/theory-of-automata"
pageToScrape = requests.get(url)

# Some urls to test:
    # https://sourcemaking.com/design_patterns
    # https://www.w3schools.com/html/
    # https://www.javatpoint.com/theory-of-automata
    # https://www.indeed.com/career-advice/career-development/types-of-networks (has bot protection)

def scrapeToText():
    # Parse the HTML content using BeautifulSoup
    soup = BeautifulSoup(pageToScrape.text, "html.parser")

    result_string = ""

    def contains_link(element):
        return element.find('a') is not None

    # Elements to exclude
    def is_valid_element(element):
        if element.name == 'a':
            return False
        if element.find_parent('a'):
            return False
        if element.find_parent('nav'):
            return False
        if element.find_parent('div', id=lambda value: value and ('menu' in value or 'footer' in value)):
            return False # Divs with id that includes 'menu' and 'footer'
        return True

    # Find all headers (h1 to h6), paragraphs, and their parent elements while excluding links and their child elements
    valid_elements = filter(is_valid_element, soup.recursiveChildGenerator())
    valid = ['h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'p', 'pre', 'li', 'span', 'div']

    # Iterate through the valid elements and print their text
    for element in valid_elements:
        if element.name and element.name in valid and not contains_link(element):
            result_string += element.text.strip() + "\n"
    
    return result_string

if check.check_availability(scrapeToText()):
    print(scrapeToText())
else:
    print("This website has bot protection in place.")

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