// Generated macro for impl_987 (impl)
macro_rules! Depcrate_io_buffered_testsimpl_987 {
() => {
// Module: crate::io::buffered::tests
// Provides: {"impl_987"}
// Dependencies: {}
impl Write for WriteRecorder { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { use crate :: str :: from_utf8 ; self . events . push (RecordedEvent :: Write (from_utf8 (buf) . unwrap () . to_string ())) ; Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { self . events . push (RecordedEvent :: Flush) ; Ok (()) } }
};
}
