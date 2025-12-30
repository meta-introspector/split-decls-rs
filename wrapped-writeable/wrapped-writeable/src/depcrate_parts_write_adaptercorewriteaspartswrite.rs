// Generated macro for CoreWriteAsPartsWrite (struct)
macro_rules! Depcrate_parts_write_adapterCoreWriteAsPartsWrite {
() => {
// Module: crate::parts_write_adapter
// Provides: {"CoreWriteAsPartsWrite"}
// Dependencies: {}
# [doc = " A wrapper around a type implementing [`fmt::Write`] that implements [`PartsWrite`]."] # [derive (Debug)] # [allow (clippy :: exhaustive_structs)] pub struct CoreWriteAsPartsWrite < W : fmt :: Write + ? Sized > (pub W) ;
};
}
