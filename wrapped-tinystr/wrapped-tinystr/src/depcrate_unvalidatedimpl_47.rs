// Generated macro for impl_47 (impl)
macro_rules! Depcrate_unvalidatedimpl_47 {
() => {
// Module: crate::unvalidated
// Provides: {"impl_47"}
// Dependencies: {}
impl < const N : usize > UnvalidatedTinyAsciiStr < N > { # [inline] # [doc = " Converts into a [`TinyAsciiStr`]. Fails if the bytes are not valid ASCII."] pub fn try_into_tinystr (self) -> Result < TinyAsciiStr < N > , ParseError > { TinyAsciiStr :: try_from_raw (self . 0) } # [inline] # [doc = " Unsafely converts into a [`TinyAsciiStr`]."] pub const fn from_utf8_unchecked (bytes : [u8 ; N]) -> Self { Self (bytes) } }
};
}
