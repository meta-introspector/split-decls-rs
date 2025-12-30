// Generated macro for cstr (macro)
macro_rules! Depcrate_cstrcstr {
() => {
// Module: crate::cstr
// Provides: {"cstr"}
// Dependencies: {}
# [doc = " A macro for [`CStr`] literals."] # [doc = ""] # [doc = " This can make passing string literals to rustix APIs more efficient, since"] # [doc = " most underlying system calls with string arguments expect NUL-terminated"] # [doc = " strings, and passing strings to rustix as `CStr`s means that rustix doesn't"] # [doc = " need to copy them into a separate buffer to NUL-terminate them."] # [doc = ""] # [doc = " In Rust ≥ 1.77, users can use [C-string literals] instead of this macro."] # [doc = ""] # [doc = " [`CStr`]: crate::ffi::CStr"] # [doc = " [C-string literals]: https://blog.rust-lang.org/2024/03/21/Rust-1.77.0.html#c-string-literals"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(feature = \"fs\")]"] # [doc = " # fn main() -> rustix::io::Result<()> {"] # [doc = " use rustix::cstr;"] # [doc = " use rustix::fs::{statat, AtFlags, CWD};"] # [doc = ""] # [doc = " let metadata = statat(CWD, cstr!(\"Cargo.toml\"), AtFlags::empty())?;"] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " # #[cfg(not(feature = \"fs\"))]"] # [doc = " # fn main() {}"] # [doc = " ```"] # [allow (unused_macros)] # [macro_export] macro_rules ! cstr { ($ str : literal) => { { :: core :: assert ! (!:: core :: iter :: Iterator :: any (& mut :: core :: primitive :: str :: bytes ($ str) , | b | b == b'\0') , "cstr argument contains embedded NUL bytes" ,) ; # [allow (unsafe_code , unused_unsafe)] { unsafe { $ crate :: ffi :: CStr :: from_bytes_with_nul_unchecked (:: core :: concat ! ($ str , "\0") . as_bytes () ,) } } } } ; }
};
}
