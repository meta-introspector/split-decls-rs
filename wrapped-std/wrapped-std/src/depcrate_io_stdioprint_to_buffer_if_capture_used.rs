// Generated macro for print_to_buffer_if_capture_used (function)
macro_rules! Depcrate_io_stdioprint_to_buffer_if_capture_used {
() => {
// Module: crate::io::stdio
// Provides: {"print_to_buffer_if_capture_used"}
// Dependencies: {}
fn print_to_buffer_if_capture_used (args : fmt :: Arguments < '_ >) -> bool { OUTPUT_CAPTURE_USED . load (Ordering :: Relaxed) && OUTPUT_CAPTURE . try_with (| s | { s . take () . map (| w | { let _ = w . lock () . unwrap_or_else (| e | e . into_inner ()) . write_fmt (args) ; s . set (Some (w)) ; }) }) == Ok (Some (())) }
};
}
