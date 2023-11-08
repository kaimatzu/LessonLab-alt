def check_availability(text):
    # Add security keywords here
    security_keywords = ["cloudflare", "captcha"]

    # Check if scraped text is a response from security checks
    text = text.lower()
    for keyword in security_keywords:
        if keyword in text:
            return False
    if not text.strip():
        return False

    # Return true if website is scrapable and ready to feed to GPT-4
    return True