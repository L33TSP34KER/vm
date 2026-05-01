mod crypto_helpers;
mod platform;
mod vm;
mod vm_runtime;

use platform::linux::main as platform_main;

fn main() {
    platform_main();
}
