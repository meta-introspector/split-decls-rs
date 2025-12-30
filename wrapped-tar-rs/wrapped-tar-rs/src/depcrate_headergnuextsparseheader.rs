// Generated macro for GnuExtSparseHeader (struct)
macro_rules! Depcrate_headerGnuExtSparseHeader {
() => {
// Module: crate::header
// Provides: {"GnuExtSparseHeader"}
// Dependencies: {}
# [doc = " Representation of the entry found to represent extended GNU sparse files."] # [doc = ""] # [doc = " When a `GnuHeader` has the `isextended` flag set to `1` then the contents of"] # [doc = " the next entry will be one of these headers."] # [repr (C)] # [allow (missing_docs)] pub struct GnuExtSparseHeader { pub sparse : [GnuSparseHeader ; GNU_EXT_SPARSE_HEADERS_COUNT] , pub isextended : [u8 ; 1] , pub padding : [u8 ; 7] , }
};
}
