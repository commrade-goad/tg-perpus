import requests

url = "http://localhost:8081/add_book"

# Define the payload (the JSON object)
payload = {
    "title": "The Five Dysfunctions of a Team",
    "author": "Patrick Lencioni",
    "desc": "The Five Dysfunctions of a Team is a business book by consultant and speaker Patrick Lencioni first published in 2002. It describes many pitfalls that teams face as they seek to 'grow together'. This book explores the fundamental causes of organizational politics and team failure. Like most of Lencioni's books, the bulk of it is written as a business fable.",
    "tagid": "4",  # Ensure this key matches what your server expects
    "year": "2002",
    "imgp": ""  # Assuming imgp is correct for your server
}

# Send the POST request
response = requests.post(url, params=payload)
# response = requests.get(url, params=payload)

# Check the response
print(response.status_code)
print(response.text)
