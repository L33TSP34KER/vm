#!/bin/bash
# Compile .asm to .bin for the custom VM
# Usage: ./compile.sh input.asm

if [ $# -ne 1 ]; then
    echo "Usage: $0 <asm_file>"
    exit 1
fi

asm_file="$1"
bin_file="${asm_file%.asm}.bin"

if [ ! -f "$asm_file" ]; then
    echo "Error: $asm_file not found"
    exit 1
fi

#!/bin/bash
# Compile .asm to .bin for the custom VM
# Usage: ./compile.sh input.asm

if [ $# -ne 1 ]; then
    echo "Usage: $0 <asm_file>"
    exit 1
fi

asm_file="$1"
bin_file="${asm_file%.asm}.bin"

if [ ! -f "$asm_file" ]; then
    echo "Error: $asm_file not found"
    exit 1
fi

# Use python to parse and generate binary
python3 -c "
import sys
import struct

# Define opcodes
op = {
    'NYAA': 0xee,
    'MEOW': 0xef,
    'NAY': 0xf0,
    'PUSH': 0xf1,
    'POP': 0xf2,
    'ADD': 0xf3,
    'SUB': 0xf4,
    'JMP': 0xf5,
    'JZ': 0xf6,
    'CALL': 0xf7,
    'RET': 0xf8,
    'LOAD': 0xf9,
    'STORE': 0xfa,
    'PRINT': 0xfb,
    'INPUT': 0xfc,
    'EQ': 0xfd,
    'CHECK': 0xfe,
    'DEBUG': 0xff
}

with open('$asm_file', 'r') as f:
    lines = f.readlines()

with open('$bin_file', 'wb') as out:
    for line in lines:
        parts = line.strip().split()
        if parts:
            mnemonic = parts[0].upper()
            if mnemonic in op:
                out.write(struct.pack('B', op[mnemonic]))
                if len(parts) > 1:
                    arg_str = parts[1]
                    if arg_str.startswith('0x'):
                        arg = int(arg_str, 16)
                    else:
                        arg = int(arg_str)
                    out.write(struct.pack('B', arg))
"

echo "Compiled $asm_file to $bin_file"

