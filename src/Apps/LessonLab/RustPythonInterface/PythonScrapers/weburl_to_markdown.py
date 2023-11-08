from bs4 import BeautifulSoup
import requests, html2text
import friendly_check as check

# Paste url here
url = "https://www.javatpoint.com/theory-of-automata"
pageToScrape = requests.get(url)

# Some urls to test:
    # https://sourcemaking.com/design_patterns
    # https://www.w3schools.com/html/
    # https://www.javatpoint.com/theory-of-automata
    # https://www.indeed.com/career-advice/career-development/types-of-networks (has bot protection)

def scrapeToMarkdown():

    html_content = pageToScrape.text

    # Parse the HTML content using BeautifulSoup
    soup = BeautifulSoup(html_content, 'html.parser')

    # Exclude specific elements (e.g., 'nav', 'a' for links)
    for tag in soup.find_all(['nav', 'a']):
        tag.extract()

    for div in soup.find_all('div', id=lambda value: value and ('menu' in value or 'footer' in value)):
        div.extract()

    # Convert the modified HTML to Markdown using html2text
    markdown_content = html2text.html2text(str(soup))

    return markdown_content

if check.check_availability(scrapeToMarkdown()):
    print(scrapeToMarkdown())
else:
    print("This website has bot protection in place.")