// Generated macro for prepare_header_link (function)
macro_rules! Depcrate_builderprepare_header_link {
() => {
// Module: crate::builder
// Provides: {"prepare_header_link"}
// Dependencies: {}
fn prepare_header_link (dst : & mut dyn Write , header : & mut Header , link_name : & Path ,) -> io :: Result < () > { if let Err (e) = header . set_link_name (link_name) { let data = path2bytes (link_name) ? ; if data . len () < header . as_old () . linkname . len () { return Err (e) ; } let header2 = prepare_header (data . len () as u64 , b'K') ; let mut data2 = data . chain (io :: repeat (0) . take (1)) ; append (dst , & header2 , & mut data2) ? ; } Ok (()) }
};
}
