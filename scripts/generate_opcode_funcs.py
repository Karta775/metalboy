import os.path
import json
import urllib.request
from argparse import ArgumentParser

OPCODES_URL = "https://raw.githubusercontent.com/lmmendes/game-boy-opcodes/master/opcodes.json"
OPCODES_FILENAME = "opcodes.json"


def get_full_instruction(values):
    instruction = values['mnemonic'] + " "
    if "operand1" in values.keys():
        instruction += values['operand1'] + " "
    if "operand2" in values.keys():
        instruction += values['operand2']
    return instruction


def create_decode_match(values):
    return '{} => Some(to_string({}, "{}", &get_operands(cpu, {}), "{}")),' \
        .format(values['addr'], values['addr'], get_full_instruction(values), values['length'], values['group'])


def create_execute_match(values):
    return '{} => execute_{}(cpu),'.format(values['addr'], values['addr'][2:])


def create_execute_func(values):
    if len(values['cycles']) > 1:
        cycles = "// Two possible CPU cycles: {}".format(values['cycles'])
    else:
        cycles = "cpu.cycles += {}".format(values['cycles'][0])
    return \
        '''fn execute_{}(cpu: &mut Cpu) {{
    op_unimplemented(cpu);
    cpu.advance_pc = {};
    {};
}} // {} [{}/{}/{}/{}]'''.format(values['addr'][2:], values['length'], cycles, get_full_instruction(values),
                                 values['flags'][0], values['flags'][1], values['flags'][2], values['flags'][3])


def main():
    if not os.path.exists(OPCODES_FILENAME):
        print("No opcodes.json is present, downloading the latest version...")
        urllib.request.urlretrieve(OPCODES_URL, OPCODES_FILENAME)

    with open(OPCODES_FILENAME) as opcodes_json:
        opcodes = json.load(opcodes_json)
        unprefixed = opcodes['unprefixed']
        cbprefixed = opcodes['cbprefixed']

        # Divide all cycle numbers by 4
        for opcode, values in unprefixed.items():
            for i in range(len(values['cycles'])):
                values['cycles'][i] = int(values['cycles'][i] / 4)

        if args.decode_matches:  # Print out every decode match (unprefixed)
            print("// Unprefixed")
            for opcode, values in unprefixed.items():
                print(create_decode_match(values))
            print("// CB prefixed")
            for opcode, values in cbprefixed.items():
                print(create_decode_match(values))
        elif args.execute_matches:  # Print out every execute match (unprefixed)
            for opcode, values in unprefixed.items():
                print(create_execute_match(values))
        elif args.execute_funcs:  # Print out every execute function stub (unprefixed)
            for opcode, values in unprefixed.items():
                print(create_execute_func(values))
        else:
            parser.print_help()


parser = ArgumentParser()
parser.add_argument("--decode-matches", action="store_true", help="Print out the decode matches")
parser.add_argument("--execute-matches", action="store_true", help="Print out the execute matches")
parser.add_argument("--execute-funcs", action="store_true", help="Print out the execute function stubs")
args = parser.parse_args()
main()
