// Generated macro for mkns (function)
macro_rules! Depcrate_callmkns {
() => {
// Module: crate::call
// Provides: {"mkns"}
// Dependencies: {}
# [doc = " Make a new scheme namespace"] pub fn mkns (schemes : & [[usize ; 2]]) -> Result < usize > { unsafe { syscall2 (SYS_MKNS , schemes . as_ptr () as usize , schemes . len ()) } }
};
}
