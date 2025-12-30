// Generated macro for impl_88 (impl)
macro_rules! Depcrate_try_writeableimpl_88 {
() => {
// Module: crate::try_writeable
// Provides: {"impl_88"}
// Dependencies: {}
impl < T > TryWriteable for WriteableAsTryWriteableInfallible < T > where T : Writeable , { type Error = Infallible ; # [inline] fn try_write_to < W : fmt :: Write + ? Sized > (& self , sink : & mut W ,) -> Result < Result < () , Infallible > , fmt :: Error > { self . 0 . write_to (sink) . map (Ok) } # [inline] fn try_write_to_parts < S : PartsWrite + ? Sized > (& self , sink : & mut S ,) -> Result < Result < () , Infallible > , fmt :: Error > { self . 0 . write_to_parts (sink) . map (Ok) } # [inline] fn writeable_length_hint (& self) -> LengthHint { self . 0 . writeable_length_hint () } # [inline] # [cfg (feature = "alloc")] fn try_write_to_string (& self) -> Result < Cow < '_ , str > , (Infallible , Cow < '_ , str >) > { Ok (self . 0 . write_to_string ()) } }
};
}
