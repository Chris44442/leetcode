
import requests

class RemoteRegMap:
    def __init__(self, host: str = "192.168.1.2", port: int = 3000):
        self.base_url = f"http://{host}:{port}"

    def read_status(self) -> int:
        r = requests.get(f"{self.base_url}/status")
        r.raise_for_status()
        return r.json()["value"]

    def write_status(self, value: int):
        r = requests.post(f"{self.base_url}/status", json={"value": value})
        r.raise_for_status()

    def read_affe(self) -> int:
        r = requests.get(f"{self.base_url}/status/affe")
        r.raise_for_status()
        return r.json()["value"]

    def write_affe(self, value: int):
        r = requests.post(f"{self.base_url}/status/affe", json={"value": value})
        r.raise_for_status()


regmap = RemoteRegMap()

print(f"Status reg: 0x{regmap.read_status():08x}")

regmap.write_status(0x12345678)

print(f"AFFE field: 0x{regmap.read_affe():04x}")

regmap.write_affe(0x4321)

print(f"Final reg: 0x{regmap.read_status():08x}")

# Read full status register
# r = requests.get("http://192.168.1.2:3000/status")
# r = requests.get("http://192.168.1.2:3000/status")
# print(f"Full status: 0x{r.json()['value']:08x}")
#
# # Write full status register
# requests.post("http://192.168.1.2:3000/status", json={"value": 0x12345678})
#
# # Read affe field
# r = requests.get("http://192.168.1.2:3000/status/affe")
# print(f"Affe field: 0x{r.json()['value']:04x}")
#
# # Write affe field
# requests.post("http://192.168.1.2:3000/status/affe", json={"value": 0x4321})
#
# r = requests.get("http://192.168.1.2:3000/status")
# print(f"Full status: 0x{r.json()['value']:08x}")
