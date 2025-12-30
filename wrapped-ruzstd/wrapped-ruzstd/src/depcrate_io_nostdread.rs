// Generated macro for Read (trait)
macro_rules! Depcrate_io_nostdRead {
() => {
// Module: crate::io_nostd
// Provides: {"Read"}
// Dependencies: {}
pub trait Read { fn read (& mut self , buf : & mut [u8]) -> Result < usize , Error > ; fn read_exact (& mut self , mut buf : & mut [u8]) -> Result < () , Error > { while ! buf . is_empty () { match self . read (buf) { Ok (0) => break , Ok (n) => { let tmp = buf ; buf = & mut tmp [n ..] ; } Err (ref e) if e . kind () == ErrorKind :: Interrupted => { } Err (e) => return Err (e) , } } if ! buf . is_empty () { Err (Error :: from (ErrorKind :: UnexpectedEof)) } else { Ok (()) } } fn read_to_end (& mut self , output : & mut alloc :: vec :: Vec < u8 >) -> Result < () , Error > { let mut buf = [0u8 ; 1024 * 16] ; loop { let bytes = self . read (& mut buf) ? ; if bytes == 0 { break ; } output . extend_from_slice (& buf [.. bytes]) ; } Ok (()) } fn take (self , limit : u64) -> Take < Self > where Self : Sized , { Take { inner : self , limit } } }
};
}
