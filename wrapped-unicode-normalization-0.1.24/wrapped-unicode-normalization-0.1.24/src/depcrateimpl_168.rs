// Generated macro for impl_168 (impl)
macro_rules! Depcrateimpl_168 {
() => {
// Module: crate
// Provides: {"impl_168"}
// Dependencies: {}
impl < 'a > UnicodeNormalization < Chars < 'a > > for & 'a str { # [inline] fn nfd (self) -> Decompositions < Chars < 'a > > { Decompositions :: new_canonical (self . chars ()) } # [inline] fn nfkd (self) -> Decompositions < Chars < 'a > > { Decompositions :: new_compatible (self . chars ()) } # [inline] fn nfc (self) -> Recompositions < Chars < 'a > > { Recompositions :: new_canonical (self . chars ()) } # [inline] fn nfkc (self) -> Recompositions < Chars < 'a > > { Recompositions :: new_compatible (self . chars ()) } # [inline] fn cjk_compat_variants (self) -> Replacements < Chars < 'a > > { replace :: new_cjk_compat_variants (self . chars ()) } # [inline] fn stream_safe (self) -> StreamSafe < Chars < 'a > > { StreamSafe :: new (self . chars ()) } }
};
}
