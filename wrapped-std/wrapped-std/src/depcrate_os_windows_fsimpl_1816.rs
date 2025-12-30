// Generated macro for impl_1816 (impl)
macro_rules! Depcrate_os_windows_fsimpl_1816 {
() => {
// Module: crate::os::windows::fs
// Provides: {"impl_1816"}
// Dependencies: {}
# [stable (feature = "metadata_ext" , since = "1.1.0")] impl MetadataExt for Metadata { fn file_attributes (& self) -> u32 { self . as_inner () . attrs () } fn creation_time (& self) -> u64 { self . as_inner () . created_u64 () } fn last_access_time (& self) -> u64 { self . as_inner () . accessed_u64 () } fn last_write_time (& self) -> u64 { self . as_inner () . modified_u64 () } fn file_size (& self) -> u64 { self . as_inner () . size () } fn volume_serial_number (& self) -> Option < u32 > { self . as_inner () . volume_serial_number () } fn number_of_links (& self) -> Option < u32 > { self . as_inner () . number_of_links () } fn file_index (& self) -> Option < u64 > { self . as_inner () . file_index () } fn change_time (& self) -> Option < u64 > { self . as_inner () . changed_u64 () } }
};
}
