"""Test the speech service """
from pykos import KOS

# Connect to KOS running on localhost at port 50051
client = KOS(ip='localhost', port=50051)

# Call the synthesize method
response = client.speech.synthesize(text="Hello, world!")

print(response)
