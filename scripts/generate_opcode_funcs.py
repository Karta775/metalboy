import os.path
import urllib.request

OPCODES_URL = "https://raw.githubusercontent.com/lmmendes/game-boy-opcodes/master/opcodes.json"
OPCODES_FILENAME = "opcodes.json"

if not os.path.exists(OPCODES_FILENAME):
    print("No opcodes.json is present, downloading the latest version...")
    urllib.request.urlretrieve(OPCODES_URL, OPCODES_FILENAME)
