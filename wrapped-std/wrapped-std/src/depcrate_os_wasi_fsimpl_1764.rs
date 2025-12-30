// Generated macro for impl_1764 (impl)
macro_rules! Depcrate_os_wasi_fsimpl_1764 {
() => {
// Module: crate::os::wasi::fs
// Provides: {"impl_1764"}
// Dependencies: {}
impl MetadataExt for fs :: Metadata { fn dev (& self) -> u64 { self . as_inner () . as_wasi () . dev } fn ino (& self) -> u64 { self . as_inner () . as_wasi () . ino } fn nlink (& self) -> u64 { self . as_inner () . as_wasi () . nlink } fn size (& self) -> u64 { self . as_inner () . as_wasi () . size } fn atim (& self) -> u64 { self . as_inner () . as_wasi () . atim } fn mtim (& self) -> u64 { self . as_inner () . as_wasi () . mtim } fn ctim (& self) -> u64 { self . as_inner () . as_wasi () . ctim } }
};
}
