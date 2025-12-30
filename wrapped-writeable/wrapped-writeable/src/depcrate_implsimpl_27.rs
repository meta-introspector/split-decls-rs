// Generated macro for impl_27 (impl)
macro_rules! Depcrate_implsimpl_27 {
() => {
// Module: crate::impls
// Provides: {"impl_27"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Writeable for String { # [inline] fn write_to < W : fmt :: Write + ? Sized > (& self , sink : & mut W) -> fmt :: Result { sink . write_str (self) } # [inline] fn writeable_length_hint (& self) -> LengthHint { LengthHint :: exact (self . len ()) } # [inline] fn writeable_borrow (& self) -> Option < & str > { Some (self) } }
};
}
