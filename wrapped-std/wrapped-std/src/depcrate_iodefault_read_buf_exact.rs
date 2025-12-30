// Generated macro for default_read_buf_exact (function)
macro_rules! Depcrate_iodefault_read_buf_exact {
() => {
// Module: crate::io
// Provides: {"default_read_buf_exact"}
// Dependencies: {}
pub (crate) fn default_read_buf_exact < R : Read + ? Sized > (this : & mut R , mut cursor : BorrowedCursor < '_ > ,) -> Result < () > { while cursor . capacity () > 0 { let prev_written = cursor . written () ; match this . read_buf (cursor . reborrow ()) { Ok (()) => { } Err (e) if e . is_interrupted () => continue , Err (e) => return Err (e) , } if cursor . written () == prev_written { return Err (Error :: READ_EXACT_EOF) ; } } Ok (()) }
};
}
