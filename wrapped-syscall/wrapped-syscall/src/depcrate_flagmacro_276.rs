// Generated macro for macro_276 (macro)
macro_rules! Depcrate_flagmacro_276 {
() => {
// Module: crate::flag
// Provides: {"macro_276"}
// Dependencies: {}
bitflags ! { pub struct MapFlags : usize { const PROT_NONE = 0x0000_0000 ; const PROT_EXEC = 0x0001_0000 ; const PROT_WRITE = 0x0002_0000 ; const PROT_READ = 0x0004_0000 ; const MAP_SHARED = 0x0001 ; const MAP_PRIVATE = 0x0002 ; const MAP_FIXED = 0x0004 ; const MAP_FIXED_NOREPLACE = 0x000C ; # [doc = " For *userspace-backed mmaps*, return from the mmap call before all pages have been"] # [doc = " provided by the scheme. This requires the scheme to be trusted, as the current context"] # [doc = " can block indefinitely, if the scheme does not respond to the page fault handler's"] # [doc = " request, as it tries to map the page by requesting it from the scheme."] # [doc = ""] # [doc = " In some cases however, such as the program loader, the data needs to be trusted as much"] # [doc = " with or without MAP_LAZY, and if so, mapping lazily will not cause insecureness by"] # [doc = " itself."] # [doc = ""] # [doc = " For kernel-backed mmaps, this flag has no effect at all. It is unspecified whether"] # [doc = " kernel mmaps are lazy or not."] const MAP_LAZY = 0x0010 ; } }
};
}
