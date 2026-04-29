// hide-macro/src/lib.rs
#[macro_export]
macro_rules! hide_call {
    // Syntax: hide_call!(function_path => arg1, arg2, arg3)
    ($($func:tt)+, $($arg:expr),* $(,)?) => {{
        (|| {
            // 1. Capture function reference & cast to integer
            let f = $($func)+;
            let ptr = f as usize;

            let mut storage = ptr;
            core::ptr::write_volatile(&mut storage, ptr);
            let resolved_addr = core::ptr::read_volatile(&storage);

            let caller: _ = unsafe { core::mem::transmute::<usize, _>(resolved_addr) };
            caller($($arg),*)
        })()
    }};
}
