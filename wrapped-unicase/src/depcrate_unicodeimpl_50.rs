// Generated macro for impl_50 (impl)
macro_rules! Depcrate_unicodeimpl_50 {
() => {
// Module: crate::unicode
// Provides: {"impl_50"}
// Dependencies: {}
impl < S : AsRef < str > > Hash for Unicode < S > { # [inline] fn hash < H : Hasher > (& self , hasher : & mut H) { let mut buf = [0 ; 4] ; for c in self . 0 . as_ref () . chars () . flat_map (| c | lookup (c)) { let len = char_to_utf8 (c , & mut buf) ; for & b in & buf [.. len] { hasher . write_u8 (b) ; } } hasher . write_u8 (0xFF) ; } }
};
}
