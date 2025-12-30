// Generated macro for try_parse_range (function)
macro_rules! Depcrate_services_fs_serve_dir_open_filetry_parse_range {
() => {
// Module: crate::services::fs::serve_dir::open_file
// Provides: {"try_parse_range"}
// Dependencies: {}
fn try_parse_range (maybe_range_ref : Option < & str > , file_size : u64 ,) -> Option < Result < Vec < RangeInclusive < u64 > > , RangeUnsatisfiableError > > { maybe_range_ref . map (| header_value | { http_range_header :: parse_range_header (header_value) . and_then (| first_pass | first_pass . validate (file_size)) }) }
};
}
