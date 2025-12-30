// Generated macro for impl_55 (impl)
macro_rules! Depcrate_parts_write_adapterimpl_55 {
() => {
// Module: crate::parts_write_adapter
// Provides: {"impl_55"}
// Dependencies: {}
impl < T : Writeable + ? Sized > Writeable for WithPart < T > { # [inline] fn write_to < W : fmt :: Write + ? Sized > (& self , sink : & mut W) -> fmt :: Result { self . writeable . write_to (sink) } # [inline] fn write_to_parts < W : PartsWrite + ? Sized > (& self , sink : & mut W) -> fmt :: Result { sink . with_part (self . part , | w | self . writeable . write_to_parts (w)) } # [inline] fn writeable_length_hint (& self) -> LengthHint { self . writeable . writeable_length_hint () } fn writeable_borrow (& self) -> Option < & str > { self . writeable . writeable_borrow () } # [inline] # [cfg (feature = "alloc")] fn write_to_string (& self) -> Cow < '_ , str > { self . writeable . write_to_string () } }
};
}
