// Generated macro for impl_28 (impl)
macro_rules! Depcrate_implsimpl_28 {
() => {
// Module: crate::impls
// Provides: {"impl_28"}
// Dependencies: {}
impl Writeable for char { # [inline] fn write_to < W : fmt :: Write + ? Sized > (& self , sink : & mut W) -> fmt :: Result { sink . write_char (* self) } # [inline] fn writeable_length_hint (& self) -> LengthHint { LengthHint :: exact (self . len_utf8 ()) } # [inline] # [cfg (feature = "alloc")] fn write_to_string (& self) -> Cow < '_ , str > { let mut s = String :: with_capacity (self . len_utf8 ()) ; s . push (* self) ; Cow :: Owned (s) } }
};
}
