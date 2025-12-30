// Generated macro for impl_26 (impl)
macro_rules! Depcrate_implsimpl_26 {
() => {
// Module: crate::impls
// Provides: {"impl_26"}
// Dependencies: {}
impl Writeable for str { # [inline] fn write_to < W : fmt :: Write + ? Sized > (& self , sink : & mut W) -> fmt :: Result { sink . write_str (self) } # [inline] fn writeable_length_hint (& self) -> LengthHint { LengthHint :: exact (self . len ()) } # [inline] fn writeable_borrow (& self) -> Option < & str > { Some (self) } }
};
}
