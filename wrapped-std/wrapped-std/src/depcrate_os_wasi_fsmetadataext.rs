// Generated macro for MetadataExt (trait)
macro_rules! Depcrate_os_wasi_fsMetadataExt {
() => {
// Module: crate::os::wasi::fs
// Provides: {"MetadataExt"}
// Dependencies: {}
# [doc = " WASI-specific extensions to [`fs::Metadata`]."] pub trait MetadataExt { # [doc = " Returns the `st_dev` field of the internal `filestat_t`"] fn dev (& self) -> u64 ; # [doc = " Returns the `st_ino` field of the internal `filestat_t`"] fn ino (& self) -> u64 ; # [doc = " Returns the `st_nlink` field of the internal `filestat_t`"] fn nlink (& self) -> u64 ; # [doc = " Returns the `st_size` field of the internal `filestat_t`"] fn size (& self) -> u64 ; # [doc = " Returns the `st_atim` field of the internal `filestat_t`"] fn atim (& self) -> u64 ; # [doc = " Returns the `st_mtim` field of the internal `filestat_t`"] fn mtim (& self) -> u64 ; # [doc = " Returns the `st_ctim` field of the internal `filestat_t`"] fn ctim (& self) -> u64 ; }
};
}
