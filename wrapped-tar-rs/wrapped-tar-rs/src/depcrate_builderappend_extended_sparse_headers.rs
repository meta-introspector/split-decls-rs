// Generated macro for append_extended_sparse_headers (function)
macro_rules! Depcrate_builderappend_extended_sparse_headers {
() => {
// Module: crate::builder
// Provides: {"append_extended_sparse_headers"}
// Dependencies: {}
# [doc = " Write extra sparse headers into `dst` for those entries that did not fit in the main header."] fn append_extended_sparse_headers (dst : & mut dyn Write , entries : & SparseEntries) -> io :: Result < () > { let mut it = entries . entries . iter () . skip (GNU_SPARSE_HEADERS_COUNT) . peekable () ; while it . peek () . is_some () { let mut ext_header = GnuExtSparseHeader :: new () ; for header_entry in ext_header . sparse . iter_mut () { if let Some (entry) = it . next () { header_entry . set_offset (entry . offset) ; header_entry . set_length (entry . num_bytes) ; } else { break ; } } ext_header . set_is_extended (it . peek () . is_some ()) ; dst . write_all (ext_header . as_bytes ()) ? ; } Ok (()) }
};
}
