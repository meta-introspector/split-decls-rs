// Generated macro for impl_29 (impl)
macro_rules! Depcrate_implsimpl_29 {
() => {
// Module: crate::impls
// Provides: {"impl_29"}
// Dependencies: {}
impl < T : Writeable + ? Sized > Writeable for & T { # [inline] fn write_to < W : fmt :: Write + ? Sized > (& self , sink : & mut W) -> fmt :: Result { (* self) . write_to (sink) } # [inline] fn write_to_parts < W : PartsWrite + ? Sized > (& self , sink : & mut W) -> fmt :: Result { (* self) . write_to_parts (sink) } # [inline] fn writeable_length_hint (& self) -> LengthHint { (* self) . writeable_length_hint () } # [inline] fn writeable_borrow (& self) -> Option < & str > { (* self) . writeable_borrow () } # [inline] # [cfg (feature = "alloc")] fn write_to_string (& self) -> Cow < '_ , str > { (* self) . write_to_string () } }
};
}
