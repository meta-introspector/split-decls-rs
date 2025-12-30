// Generated macro for impl_158 (impl)
macro_rules! Depcrate_headerimpl_158 {
() => {
// Module: crate::header
// Provides: {"impl_158"}
// Dependencies: {}
impl GnuExtSparseHeader { # [doc = " Crates a new zero'd out sparse header entry."] pub fn new () -> GnuExtSparseHeader { unsafe { mem :: zeroed () } } # [doc = " Returns a view into this header as a byte array."] pub fn as_bytes (& self) -> & [u8 ; BLOCK_SIZE as usize] { debug_assert_eq ! (mem :: size_of_val (self) , BLOCK_SIZE as usize) ; unsafe { mem :: transmute (self) } } # [doc = " Returns a view into this header as a byte array."] pub fn as_mut_bytes (& mut self) -> & mut [u8 ; BLOCK_SIZE as usize] { debug_assert_eq ! (mem :: size_of_val (self) , BLOCK_SIZE as usize) ; unsafe { mem :: transmute (self) } } # [doc = " Returns a slice of the underlying sparse headers."] # [doc = ""] # [doc = " Some headers may represent empty chunks of both the offset and numbytes"] # [doc = " fields are 0."] pub fn sparse (& self) -> & [GnuSparseHeader ; 21] { & self . sparse } # [doc = " Same as `sparse` but mutable version."] pub fn sparse_mut (& mut self) -> & mut [GnuSparseHeader ; 21] { & mut self . sparse } # [doc = " Indicates if another sparse header should be following this one."] pub fn is_extended (& self) -> bool { self . isextended [0] == 1 } # [doc = " Sets whether another sparse header should be following this one."] pub fn set_is_extended (& mut self , is_extended : bool) { self . isextended [0] = if is_extended { 1 } else { 0 } ; } }
};
}
