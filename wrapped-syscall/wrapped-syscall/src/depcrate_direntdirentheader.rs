// Generated macro for DirentHeader (struct)
macro_rules! Depcrate_direntDirentHeader {
() => {
// Module: crate::dirent
// Provides: {"DirentHeader"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , Default)] # [repr (packed)] pub struct DirentHeader { pub inode : u64 , # [doc = " A filesystem-specific opaque value used to uniquely identify directory entries. This value,"] # [doc = " in the last returned entry from a SYS_GETDENTS invocation, shall be passed to the next"] # [doc = " call."] pub next_opaque_id : u64 , pub record_len : u16 , # [doc = " A `DirentKind`."] # [doc = ""] # [doc = " May not be directly available (Unspecified), and if so needs to be looked using fstat."] pub kind : u8 , }
};
}
