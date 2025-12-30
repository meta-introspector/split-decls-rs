// Generated macro for ZIP64_ENTRY_THR (const)
macro_rules! Depcrate_specZIP64_ENTRY_THR {
() => {
// Module: crate::spec
// Provides: {"ZIP64_ENTRY_THR"}
// Dependencies: {}
# [doc = " The number of entries within a single zip necessary to allocate a zip64 central"] # [doc = " directory record."] # [doc = ""] # [doc = " If more than this number of entries is written to a [`ZipWriter`], then [`ZipWriter::finish()`]"] # [doc = " will write out extra zip64 data to the end of the zip file."] pub const ZIP64_ENTRY_THR : usize = u16 :: MAX as usize ;
};
}
