// Generated macro for impl_193 (impl)
macro_rules! Depcrate_tls_streamimpl_193 {
() => {
// Module: crate::tls_stream
// Provides: {"impl_193"}
// Dependencies: {}
impl < S > BufRead for TlsStream < S > where S : Read + Write , { fn fill_buf (& mut self) -> io :: Result < & [u8] > { while self . get_buf () . is_empty () { if self . initialize () ? . is_none () { break ; } if self . needs_read > 0 { if self . read_in () ? == 0 { break ; } self . needs_read = 0 ; } let eof = self . decrypt () ? ; if eof { break ; } } Ok (self . get_buf ()) } fn consume (& mut self , amt : usize) { let pos = self . dec_in . position () + amt as u64 ; assert ! (pos <= self . dec_in . get_ref () . len () as u64) ; self . dec_in . set_position (pos) ; } }
};
}
