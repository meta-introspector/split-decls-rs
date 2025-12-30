// Generated macro for default_read_exact (function)
macro_rules! Depcrate_iodefault_read_exact {
() => {
// Module: crate::io
// Provides: {"default_read_exact"}
// Dependencies: {}
pub (crate) fn default_read_exact < R : Read + ? Sized > (this : & mut R , mut buf : & mut [u8]) -> Result < () > { while ! buf . is_empty () { match this . read (buf) { Ok (0) => break , Ok (n) => { buf = & mut buf [n ..] ; } Err (ref e) if e . is_interrupted () => { } Err (e) => return Err (e) , } } if ! buf . is_empty () { Err (Error :: READ_EXACT_EOF) } else { Ok (()) } }
};
}
