// Generated macro for impl_169 (impl)
macro_rules! Depcrateimpl_169 {
() => {
// Module: crate
// Provides: {"impl_169"}
// Dependencies: {}
impl UnicodeNormalization < option :: IntoIter < char > > for char { # [inline] fn nfd (self) -> Decompositions < option :: IntoIter < char > > { Decompositions :: new_canonical (Some (self) . into_iter ()) } # [inline] fn nfkd (self) -> Decompositions < option :: IntoIter < char > > { Decompositions :: new_compatible (Some (self) . into_iter ()) } # [inline] fn nfc (self) -> Recompositions < option :: IntoIter < char > > { Recompositions :: new_canonical (Some (self) . into_iter ()) } # [inline] fn nfkc (self) -> Recompositions < option :: IntoIter < char > > { Recompositions :: new_compatible (Some (self) . into_iter ()) } # [inline] fn cjk_compat_variants (self) -> Replacements < option :: IntoIter < char > > { Replacements :: new_cjk_compat_variants (Some (self) . into_iter ()) } # [inline] fn stream_safe (self) -> StreamSafe < option :: IntoIter < char > > { StreamSafe :: new (Some (self) . into_iter ()) } }
};
}
