// Generated macro for stack_buffer_copy (function)
macro_rules! Depcrate_io_copystack_buffer_copy {
() => {
// Module: crate::io::copy
// Provides: {"stack_buffer_copy"}
// Dependencies: {}
pub fn stack_buffer_copy < R : Read + ? Sized , W : Write + ? Sized > (reader : & mut R , writer : & mut W ,) -> Result < u64 > { let buf : & mut [_] = & mut [MaybeUninit :: uninit () ; DEFAULT_BUF_SIZE] ; let mut buf : BorrowedBuf < '_ > = buf . into () ; let mut len = 0 ; loop { match reader . read_buf (buf . unfilled ()) { Ok (()) => { } Err (e) if e . is_interrupted () => continue , Err (e) => return Err (e) , } ; if buf . filled () . is_empty () { break ; } len += buf . filled () . len () as u64 ; writer . write_all (buf . filled ()) ? ; buf . clear () ; } Ok (len) }
};
}
