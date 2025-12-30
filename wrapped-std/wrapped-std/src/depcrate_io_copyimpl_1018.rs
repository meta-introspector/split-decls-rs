// Generated macro for impl_1018 (impl)
macro_rules! Depcrate_io_copyimpl_1018 {
() => {
// Module: crate::io::copy
// Provides: {"impl_1018"}
// Dependencies: {}
impl < I : Write + ? Sized > BufferedWriterSpec for BufWriter < I > { fn buffer_size (& self) -> usize { self . capacity () } fn copy_from < R : Read + ? Sized > (& mut self , reader : & mut R) -> Result < u64 > { if self . capacity () < DEFAULT_BUF_SIZE { return stack_buffer_copy (reader , self) ; } let mut len = 0 ; let mut init = 0 ; loop { let buf = self . buffer_mut () ; let mut read_buf : BorrowedBuf < '_ > = buf . spare_capacity_mut () . into () ; unsafe { read_buf . set_init (init) ; } if read_buf . capacity () >= DEFAULT_BUF_SIZE { let mut cursor = read_buf . unfilled () ; match reader . read_buf (cursor . reborrow ()) { Ok (()) => { let bytes_read = cursor . written () ; if bytes_read == 0 { return Ok (len) ; } init = read_buf . init_len () - bytes_read ; len += bytes_read as u64 ; unsafe { buf . set_len (buf . len () + bytes_read) } ; } Err (ref e) if e . is_interrupted () => { } Err (e) => return Err (e) , } } else { init += buf . len () ; self . flush_buf () ? ; } } } }
};
}
