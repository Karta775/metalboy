import os.path
import json
import urllib.request

OPCODES_URL = "https://raw.githubusercontent.com/lmmendes/game-boy-opcodes/master/opcodes.json"
OPCODES_FILENAME = "opcodes.json"


def create_decode_match(values):
    return '{} => Some(to_string({}, "{}", {}, "{}", "{}")),' \
        .format(values['addr'], values['addr'], values['mnemonic'], values['length'], values['cycles'], values['group'])


def main():
    if not os.path.exists(OPCODES_FILENAME):
        print("No opcodes.json is present, downloading the latest version...")
        urllib.request.urlretrieve(OPCODES_URL, OPCODES_FILENAME)

    with open(OPCODES_FILENAME) as opcodes_json:
        opcodes = json.load(opcodes_json)
        unprefixed = opcodes['unprefixed']
        cbprefixed = opcodes['cbprefixed']

        # Print out every decode match (unprefixed)
        for opcode, values in unprefixed.items():
            print(create_decode_match(values))


main()
