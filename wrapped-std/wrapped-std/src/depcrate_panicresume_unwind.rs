// Generated macro for resume_unwind (function)
macro_rules! Depcrate_panicresume_unwind {
() => {
// Module: crate::panic
// Provides: {"resume_unwind"}
// Dependencies: {}
# [doc = " Triggers a panic without invoking the panic hook."] # [doc = ""] # [doc = " This is designed to be used in conjunction with [`catch_unwind`] to, for"] # [doc = " example, carry a panic across a layer of C code."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " Note that panics in Rust are not always implemented via unwinding, but they"] # [doc = " may be implemented by aborting the process. If this function is called when"] # [doc = " panics are implemented this way then this function will abort the process,"] # [doc = " not trigger an unwind."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use std::panic;"] # [doc = ""] # [doc = " let result = panic::catch_unwind(|| {"] # [doc = "     if 1 != 2 {"] # [doc = "         panic!(\"oh no!\");"] # [doc = "     }"] # [doc = " });"] # [doc = ""] # [doc = " if let Err(err) = result {"] # [doc = "     panic::resume_unwind(err);"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "resume_unwind" , since = "1.9.0")] pub fn resume_unwind (payload : Box < dyn Any + Send >) -> ! { panicking :: resume_unwind (payload) }
};
}
