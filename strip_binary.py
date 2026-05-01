#!/usr/bin/env python3
"""
strip_binary.py - Aggressively strip ALL debug/identifying strings from a Rust ELF binary.

Nukes every byte pattern that could reveal source paths, crate names, Rust internals,
or any information not programmatically required by the binary at runtime.

Usage:
    python3 strip_binary.py <binary_path> [--output <output_path>]
"""

import argparse
import re
import shutil
import subprocess
import sys
import os

PRESERVED_SUBSTRINGS = [
    b'/dev/random',
    b'CRYPTIFY_KEY',
    b'EBU5XXFyG',
    b'0aXXBv4ZZ',
    b'Q8FeXXe&W9Xdt1XXW6LB',
    b'AS6lXXVAd1oXXg6q#fm1',
    b'StreamCipherError',
    b'SystemTimeError',
    b'/lib64/ld-linux-x86-64.so.2',
]

BINARY_PATTERNS_TO_NUKE = [
    re.compile(rb'vm[\-_]core[\-/\\]'),
    re.compile(rb'vm[\-_]bin[\-/\\]'),
    re.compile(rb'crypto_helpers'),
    re.compile(rb'vm_runtime'),
    re.compile(rb'platform/linux'),
    re.compile(rb'platform/macos'),
    re.compile(rb'src/vm/OP'),
    re.compile(rb'src/vm/RAM'),
    re.compile(rb'src/vm/VM'),
    re.compile(rb'/rustc/[0-9a-f]{20,}'),
    re.compile(rb'/rust/deps/'),
    re.compile(rb'library/(std|core|alloc|std_macros)/'),
    re.compile(rb'src/(alloc|collections|cell|ffi|fmt|io|num|ops|path|raw_vec|rt|slice|str|sync|sys|thread|time|unicode|vec|backtrace|panicking|string)/'),
    re.compile(rb'addr2line[^/\x00]*/src/'),
    re.compile(rb'gimli[^/\x00]*/src/'),
    re.compile(rb'hashbrown[^/\x00]*/src/'),
    re.compile(rb'miniz_oxide[^/\x00]*/src/'),
    re.compile(rb'rustc-demangle[^/\x00]*/src/'),
    re.compile(rb'cipher[^/\x00]*/src/'),
    re.compile(rb'/home/[^\x00]{4,}'),
    re.compile(rb'/cargo/'),
    re.compile(rb'\.rs[\x00:/\\]'),
    re.compile(rb'::core::'),
    re.compile(rb'::alloc::'),
    re.compile(rb'::std::'),
    re.compile(rb'::vm_core::'),
    re.compile(rb'::vm_bin::'),
    re.compile(rb'::zeroize::'),
    re.compile(rb'::cryptify::'),
    re.compile(rb'::chacha20::'),
    re.compile(rb'::cipher::'),
    re.compile(rb'::libc::'),
    re.compile(rb'::hide_macro::'),
    re.compile(rb'_ZN[0-9]'),
    re.compile(rb'_RNv'),
    re.compile(rb'__rust_'),
    re.compile(rb'rust_begin_unwind'),
    re.compile(rb'rust_eh_personality'),
    re.compile(rb'rust_panic'),
    re.compile(rb'__llvm_'),
    re.compile(rb'GNU C\b'),
    re.compile(rb'GCC:\s'),
    re.compile(rb'Gentoo\b'),
    re.compile(rb'Linker:\s+LLD'),
    re.compile(rb'\.debug_\w+'),
    re.compile(rb'\.note\.gnu'),
    re.compile(rb'\.note\.ABI'),
    re.compile(rb'\.note\.package'),
    re.compile(rb'\.comment\x00'),
    re.compile(rb'\.gcc_except'),
    re.compile(rb'\.eh_frame'),
    re.compile(rb'\.tm_clone'),
    re.compile(rb'\.symtab\x00'),
    re.compile(rb'\.strtab\x00'),
    re.compile(rb'\.gnu\.hash\x00'),
    re.compile(rb'\.gnu\.version\x00'),
    re.compile(rb'\.gnu\.debuglink'),
    re.compile(rb'\.gnu_debugaltlinm'),
    re.compile(rb'\.zdebug_'),
    re.compile(rb'/usr/lib/debug'),
    re.compile(rb'/usr/lib/\x00'),
    re.compile(rb'/proc/self'),
    re.compile(rb'expand 32-byte k'),
    re.compile(rb'vm[\-_]core'),
    re.compile(rb'vm[\-_]bin'),
    re.compile(rb'Q8FeXXe&W9Xdt1XXW6LB'),
    re.compile(rb'AS6lXXVAd1oXXg6q#fm1'),
    re.compile(rb'assertion failed'),
    re.compile(rb'Formatting argument out of range'),
    re.compile(rb'idx < CAPACITY'),
]

STRING_PATTERNS_TO_NUKE = [
    b'borrowed',
    b'already_borrowed',
    b'RefCell',
    b'panic',
    b'PANIC',
    b'backtrace',
    b'unreachable',
    b'overflow',
    b'assertion',
    b'capacity',
    b'allocation',
    b'bytes failed',
    b'fatal runtime',
    b'deadlock',
    b'internal error',
    b'copy_from_slice',
    b'slice length',
    b'out of bounds',
    b'out of range',
    b'char boundary',
    b'OsStr boundary',
    b'recursion limit',
    b'Formatting argument',
    b'Hash table',
    b'user-provided comparison',
    b'memory allocat',
    b'G bytes failed',
    b'str::from_utf8',
    b'called `Option::unwrap',
    b'called `Result::unwrap',
    b'host unreachable',
    b'connection reset',
    b'connection refused',
    b'network unreachable',
    b'connection aborted',
    b'not connected',
    b'address in use',
    b'AddrNotAvailable',
    b'network down',
    b'broken pipe',
    b'entity already exists',
    b'operation would block',
    b'read-only filesystem',
    b'filesystem loop',
    b'stale network file handle',
    b'invalid input parameter',
    b'invalid data',
    b'timed out',
    b'write zero',
    b'seek on unseekable',
    b'quota exceeded',
    b'file too large',
    b'resource busy',
    b'executable file busy',
    b'cross-device link',
    b'too many links',
    b'argument list too long',
    b'operation interrupted',
    b'in progress',
    b'uncategorized error',
    b'PermissionDenied',
    b'NotFound',
    b'TimedOut',
    b'Custom',
    b'invalid syntax',
    b'invalid filename',
    b'no storage space',
    b'skipping backtrace',
    b'hash table capacity',
    b'not a directory',
    b'is a directory',
    b'directory not empty',
    b'stack overflow',
    b'alternative stack',
    b'guard page',
    b'alternate signal stack',
    b'signal handler',
    b'thread ',
    b'panicked',
]

SECTIONS_TO_REMOVE = [
    '.debug_abbrev', '.debug_addr', '.debug_aranges', '.debug_frame',
    '.debug_info', '.debug_line', '.debug_line_str', '.debug_loclists',
    '.debug_macinfo', '.debug_names', '.debug_pubnames', '.debug_pubtypes',
    '.debug_ranges', '.debug_rnglists', '.debug_str', '.debug_str_offsets',
    '.debug_types', '.comment', '.note.gnu.build-id', '.note.ABI-tag',
    '.note.package', '.note.gnu.gold-version', '.gcc_except_table',
    '.eh_frame', '.eh_frame_hdr', '.tm_clone_table', '.symtab', '.strtab',
]


def is_near_preserved(data, start, end):
    window_start = max(0, start - 32)
    window_end = min(len(data), end + 32)
    window = data[window_start:window_end]
    for ps in PRESERVED_SUBSTRINGS:
        if ps in window:
            return True
    return False


def nuke_all(data):
    count = 0
    for pat in BINARY_PATTERNS_TO_NUKE:
        new_data = b''
        last_end = 0
        for m in pat.finditer(data):
            s, e = m.start(), m.end()
            if is_near_preserved(data, s, e):
                continue
            new_data += data[last_end:s]
            new_data += b'\x00' * (e - s)
            last_end = e
            count += 1
        new_data += data[last_end:]
        data = new_data

    i = 0
    while i < len(data):
        if data[i] == 0:
            i += 1
            continue
        end = i
        while end < len(data) and data[end] != 0:
            end += 1
        string = data[i:end]
        should_nuke = False
        for pat in STRING_PATTERNS_TO_NUKE:
            if pat in string and not is_near_preserved(data, i, end):
                should_nuke = True
                break
        if should_nuke:
            data = data[:i] + (b'\x00' * (end - i)) + data[end:]
            count += 1
        i = end + 1

    return data, count


def process_binary(input_path, output_path=None):
    if output_path is None:
        output_path = input_path

    if input_path != output_path:
        shutil.copy2(input_path, output_path)

    binary_path = output_path
    original_size = os.path.getsize(binary_path)

    print(f"[1/4] Stripping symbols and removing sections...")
    if shutil.which('strip'):
        subprocess.run(['strip', '--strip-all', '--remove-section=.comment',
                        '--remove-section=.note.gnu.build-id',
                        '--remove-section=.note.ABI-tag', binary_path],
                       check=True, capture_output=True)

    with open(binary_path, 'rb') as f:
        data = f.read()

    try:
        from elftools.elf.elffile import ELFFile
        from io import BytesIO
        elf = ELFFile(BytesIO(data))
        section_names = [s.name for s in elf.iter_sections()]
    except Exception:
        section_names = []

    remove_args = []
    for sec in SECTIONS_TO_REMOVE:
        if sec in section_names:
            remove_args.extend(['--remove-section', sec])
    for name in section_names:
        for prefix in ['.debug_', '.note.']:
            if name.startswith(prefix):
                remove_args.extend(['--remove-section', name])

    if remove_args and shutil.which('objcopy'):
        subprocess.run(['objcopy'] + remove_args + [binary_path, binary_path],
                       check=True, capture_output=True)

    print(f"[2/4] Nuking identifying byte patterns (pass 1)...")
    with open(binary_path, 'rb') as f:
        data = f.read()
    data, count1 = nuke_all(data)
    with open(binary_path, 'wb') as f:
        f.write(data)
    print(f"  Nulled {count1} patterns")

    print(f"[3/4] Nuking identifying byte patterns (pass 2 - catch cascading)...")
    with open(binary_path, 'rb') as f:
        data = f.read()
    data, count2 = nuke_all(data)
    with open(binary_path, 'wb') as f:
        f.write(data)
    print(f"  Nulled {count2} additional patterns")

    print(f"[4/4] Final strip pass...")
    if shutil.which('strip'):
        subprocess.run(['strip', '--strip-all', binary_path],
                       check=True, capture_output=True)

    final_size = os.path.getsize(binary_path)
    removed = original_size - final_size
    pct = 100 * removed // original_size if original_size else 0
    print(f"\nDone! Output: {binary_path}")
    print(f"  Original: {original_size:,} bytes")
    print(f"  Final:    {final_size:,} bytes")
    print(f"  Removed:  {removed:,} bytes ({pct}%)")
    print(f"  Total nuked: {count1 + count2} patterns")


def main():
    parser = argparse.ArgumentParser(description='Strip ALL debug/identifying strings from a Rust ELF binary')
    parser.add_argument('binary', help='Path to the ELF binary to strip')
    parser.add_argument('--output', '-o', help='Output path (default: modify in-place)')

    args = parser.parse_args()

    if not os.path.isfile(args.binary):
        print(f"Error: {args.binary} not found", file=sys.stderr)
        sys.exit(1)

    process_binary(args.binary, args.output)


if __name__ == '__main__':
    main()