// Generated macro for prepare_header_path (function)
macro_rules! Depcrate_builderprepare_header_path {
() => {
// Module: crate::builder
// Provides: {"prepare_header_path"}
// Dependencies: {}
fn prepare_header_path (dst : & mut dyn Write , header : & mut Header , path : & Path) -> io :: Result < () > { if let Err (e) = header . set_path (path) { let data = path2bytes (path) ? ; let max = header . as_old () . name . len () ; if data . len () < max { return Err (e) ; } let header2 = prepare_header (data . len () as u64 , b'L') ; let mut data2 = data . chain (io :: repeat (0) . take (1)) ; append (dst , & header2 , & mut data2) ? ; let truncated = match str :: from_utf8 (& data [.. max]) { Ok (s) => s , Err (e) => str :: from_utf8 (& data [.. e . valid_up_to ()]) . unwrap () , } ; header . set_truncated_path_for_gnu_header (truncated) ? ; } Ok (()) }
};
}
