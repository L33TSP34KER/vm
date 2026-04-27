#!/bin/bash
# LLVM obfuscation script for vm_crackme
# This script applies LLVM obfuscation passes to the compiled binary
# 
# Requirements: LLVM/Clang with obfuscation passes (ollama-obfuscator)
# For standard LLVM, this provides basic CFG flattening
#
# Usage: ./obfuscate.sh <binary_path>

set -e

BINARY="${1:-target/release/vm_crackme}"

if [ ! -f "$BINARY" ]; then
    echo "Error: Binary not found: $BINARY"
    echo "Build first with: cargo build --release"
    exit 1
fi

BACKUP="${BINARY}.backup"
cp "$BINARY" "$BACKUP"

echo "Applying LLVM obfuscation to $BINARY..."

# Check for OLLVM opt (from obfuscator-llvm)
OLLVM_OPT=""
if command -v opt &> /dev/null; then
    if opt --help 2>&1 | grep -q "fla"; then
        OLLVM_OPT="opt"
    fi
fi

if [ -n "$OLLVM_OPT" ]; then
    echo "Using OLLVM opt for advanced obfuscation..."
    # Apply obfuscation passes: flattening, bogus control flow, substitution
    $OLLVM_OPT -fla -bcf -sub "$BINARY" -o "${BINARY}.obf" 2>/dev/null || true
    
    if [ -f "${BINARY}.obf" ]; then
        mv "${BINARY}.obf" "$BINARY"
    fi
else
    echo "OLLVM not found, using basic optimizations..."
    # Try to apply basic optimizations that provide some obfuscation
    if command -v opt &> /dev/null; then
        opt -O3 -strip-debug "$BINARY" -o "${BINARY}.opt" 2>/dev/null || true
        
        if [ -f "${BINARY}.opt" ]; then
            mv "${BINARY}.opt" "$BINARY"
        fi
    fi
fi

# Additional obfuscation using strip and objcopy
if command -v strip &> /dev/null; then
    strip --strip-all --remove-section=.comment "$BINARY" 2>/dev/null || true
fi

if command -v objcopy &> /dev/null; then
    objcopy --strip-all "$BINARY" 2>/dev/null || true
fi

echo "Obfuscation complete!"
echo "Original binary backed up to: $BACKUP"

# Show file info
ls -lh "$BINARY"