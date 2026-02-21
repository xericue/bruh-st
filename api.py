import requests

response = requests.get("https://randomfox.ca/floof/")
if response.status_code == 200:
    data = response.json()
    print(f'open this link for an awesome fox! {data['link']}')
else:
    print(f"no fox for you! failure: {response.status_code}")
