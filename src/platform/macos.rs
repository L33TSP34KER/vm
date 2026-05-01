use crate::vm_runtime;

pub fn main() {
    unsafe { vm_runtime::real_code() };
}
