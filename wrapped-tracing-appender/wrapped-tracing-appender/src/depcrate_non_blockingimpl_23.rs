// Generated macro for impl_23 (impl)
macro_rules! Depcrate_non_blockingimpl_23 {
() => {
// Module: crate::non_blocking
// Provides: {"impl_23"}
// Dependencies: {}
impl std :: io :: Write for NonBlocking { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let buf_size = buf . len () ; if self . is_lossy { if self . channel . try_send (Msg :: Line (buf . to_vec ())) . is_err () { self . error_counter . incr_saturating () ; } } else { return match self . channel . send (Msg :: Line (buf . to_vec ())) { Ok (_) => Ok (buf_size) , Err (_) => Err (io :: Error :: from (io :: ErrorKind :: Other)) , } ; } Ok (buf_size) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } # [inline] fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { self . write (buf) . map (| _ | ()) } }
};
}
