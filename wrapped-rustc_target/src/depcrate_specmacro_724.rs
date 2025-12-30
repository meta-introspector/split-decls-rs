// Generated macro for macro_724 (macro)
macro_rules! Depcrate_specmacro_724 {
() => {
// Module: crate::spec
// Provides: {"macro_724"}
// Dependencies: {}
bitflags :: bitflags ! { # [derive (Clone , Copy , PartialEq , Eq , Default)] # [doc = " The `-C link-self-contained` components that can individually be enabled or disabled."] pub struct LinkSelfContainedComponents : u8 { # [doc = " CRT objects (e.g. on `windows-gnu`, `musl`, `wasi` targets)"] const CRT_OBJECTS = 1 << 0 ; # [doc = " libc static library (e.g. on `musl`, `wasi` targets)"] const LIBC = 1 << 1 ; # [doc = " libgcc/libunwind (e.g. on `windows-gnu`, `fuchsia`, `fortanix`, `gnullvm` targets)"] const UNWIND = 1 << 2 ; # [doc = " Linker, dlltool, and their necessary libraries (e.g. on `windows-gnu` and for `rust-lld`)"] const LINKER = 1 << 3 ; # [doc = " Sanitizer runtime libraries"] const SANITIZERS = 1 << 4 ; # [doc = " Other MinGW libs and Windows import libs"] const MINGW = 1 << 5 ; } }
};
}
