// Generated macro for impl_22 (impl)
macro_rules! Depcrate_asciiimpl_22 {
() => {
// Module: crate::ascii
// Provides: {"impl_22"}
// Dependencies: {}
impl < const N : usize > FromStr for TinyAsciiStr < N > { type Err = ParseError ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: try_from_str (s) } }
};
}
