# vm-crackme

A heavily obfuscated password-checking crackme built as a custom Virtual Machine in Rust. The program prompts for a password, runs it through a custom VM with a hand-crafted instruction set, and grants or denies access based on the result. It employs extensive anti-debugging, anti-analysis, and binary-stripping measures to resist reverse engineering.

## Architecture

```
vm/
├── vm-core/       Core VM runtime (opcodes, RAM, executor)
├── vm-bin/        Binary crate — entry point, crypto bootstrap, platform code
├── vm-db/         Debugger stub (unimplemented)
├── hide-macro/    `hide_call!` macro for indirect function calls
├── compile.sh     Assembler: .asm → .bin bytecode
├── strip_binary.py  Post-build binary sanitizer
└── password.asm   VM assembly source for the password challenge
```

### vm-core

Defines the VM runtime:

- **OpCode enum** — 19 opcodes in the `0xED..0xFF` range (`FN`, `Nyaa`, `Meow`, `Nay`, `Push`, `Pop`, `Add`, `Sub`, `Jmp`, `Jz`, `Call`, `Ret`, `Load`, `Store`, `Print`, `Input`, `Eq`, `Check`, `Debug`)
- **RAM** — XOR-obfuscated virtual memory using `Zeroizing<Vec<u8>>`; all indices XOR'd with a time-derived key
- **VM** — Fetch-decode-execute loop with anti-debug timing checks and function table dispatch
- **Key fragments** — `KEY_A` (at `vm/mod.rs`) and `KEY_B` (at `vm/OP.rs`) hold the ChaCha20 key halves with junk byte injection; `NONCE_B` (at `vm/VM.rs`) holds the nonce suffix

### vm-bin

The actual executable:

- **`build.rs`** — Encrypts `password.bin` with ChaCha20 at compile time, embedding the ciphertext into the binary
- **`crypto_helpers.rs`** — Reconstructs the ChaCha20 key/nonce at runtime from fragments scattered across crates, stripping junk bytes
- **`platform/linux.rs`** — Linux entry point: captures RSP via `.init_array`, sets up a `SIGEMT` handler that dispatches through an inline-asm trampoline, then busy-loops on `/dev/random` until seeing byte `127`
- **`platform/macos.rs`** — macOS entry point: calls `real_code()` directly
- **`vm_runtime.rs`** — Decrypts the embedded bytecode, feeds it (and the encrypted decoy) into the VM, then runs

### hide-macro

`hide_call!` — Obfuscates function calls by capturing a function pointer as `usize`, storing it through `write_volatile`/`read_volatile`, then `transmute`-ing it back to a callable. Defeats naive static call-graph analysis.

### vm-db

Placeholder debugger interface (`DebugVm`, `Breakpoint`, `MemoryView`, `Session`). All methods are `todo!()`.

## Build & Run

```bash
# 1. Assemble the VM bytecode
./compile.sh password.asm

# 2. Build the binary (build.rs encrypts password.bin)
cargo build --release

# 3. (Optional) Strip identifying strings from the binary
python3 strip_binary.py target/release/vm-bin

# 4. Run
./target/release/vm-bin
```

## Anti-Reverse-Engineering Features

| Feature | Implementation |
|---------|---------------|
| XOR-obfuscated RAM | All indices/values XOR'd with time-derived key; `Zeroizing` wipe on drop |
| ChaCha20 bytecode encryption | Bytecode encrypted at build time, decrypted at runtime |
| Split key construction | Key/nonce fragments scattered across 3 crates with junk byte injection |
| Timing anti-debug | >500ms startup check corrupts RAM; >1s per-op check corrupts RAM |
| SIGEMT trampoline | Linux entry dispatches through signal handler + inline-asm indirect jump |
| `/dev/random` entry delay | Busy-loop until random byte 127; encrypted data overwritten with `/dev/random` if debugger detected |
| Opaque predicates | `cryptify::flow_stmt!()` junk control flow throughout |
| String encryption | `cryptify::encrypt_string!` for debug messages |
| Function call obfuscation | `hide_call!` volatile pointer indirection |
| Binary stripping | `strip_binary.py` removes all Rust/crate/dependency strings |
| LTO + strip + panic=abort | Minimal reverse-engineering surface |

## VM Instruction Set

| Opcode | Hex | Operand | Description |
|--------|-----|---------|-------------|
| FN     | ED  | id      | Function definition |
| Nyaa   | EE  | —       | Print "Nyaa" (decoy) |
| Meow   | EF  | —       | Print "Meow" (decoy) |
| Nay    | F0  | —       | Exit process |
| Push   | F1  | byte    | Push byte onto stack |
| Pop    | F2  | —       | Pop from stack |
| Add    | F3  | —       | Stack addition |
| Sub    | F4  | —       | Stack subtraction |
| Jmp    | F5  | addr    | Unconditional jump |
| Jz     | F6  | addr    | Jump if top-of-stack is zero |
| Call   | F7  | id      | Call function by table ID |
| Ret    | F8  | —       | Return from call |
| Load   | F9  | —       | Load from RAM to stack |
| Store  | FA  | addr    | Store from stack to RAM |
| Print  | FB  | idx     | Print stack[idx] as char |
| Input  | FC  | —       | Read stdin onto stack |
| Eq     | FD  | —       | Compare top two stack elements |
| Check  | FE  | —       | Password verification gate |
| Debug  | FF  | —       | Dump VM state |