// Generated macro for prepare_header_sparse (function)
macro_rules! Depcrate_builderprepare_header_sparse {
() => {
// Module: crate::builder
// Provides: {"prepare_header_sparse"}
// Dependencies: {}
fn prepare_header_sparse (file : & mut fs :: File , stat : & fs :: Metadata , header : & mut Header ,) -> io :: Result < Option < SparseEntries > > { let entries = match find_sparse_entries (file , stat) ? { Some (entries) => entries , _ => return Ok (None) , } ; header . set_entry_type (EntryType :: GNUSparse) ; header . set_size (entries . on_disk_size) ; let gnu_header = & mut header . as_gnu_mut () . unwrap () ; gnu_header . set_real_size (entries . size ()) ; for (entry , header_entry) in std :: iter :: zip (& entries . entries , & mut gnu_header . sparse) { header_entry . set_offset (entry . offset) ; header_entry . set_length (entry . num_bytes) ; } gnu_header . set_is_extended (entries . entries . len () > gnu_header . sparse . len ()) ; Ok (Some (entries)) }
};
}
