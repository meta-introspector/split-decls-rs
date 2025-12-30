// Generated macro for impl_85 (impl)
macro_rules! Depcrate_try_writeableimpl_85 {
() => {
// Module: crate::try_writeable
// Provides: {"impl_85"}
// Dependencies: {}
impl < T > Writeable for TryWriteableInfallibleAsWriteable < T > where T : TryWriteable < Error = Infallible > , { # [inline] fn write_to < W : fmt :: Write + ? Sized > (& self , sink : & mut W) -> fmt :: Result { match self . 0 . try_write_to (sink) { Ok (Ok (())) => Ok (()) , Ok (Err (infallible)) => match infallible { } , Err (e) => Err (e) , } } # [inline] fn write_to_parts < S : PartsWrite + ? Sized > (& self , sink : & mut S) -> fmt :: Result { match self . 0 . try_write_to_parts (sink) { Ok (Ok (())) => Ok (()) , Ok (Err (infallible)) => match infallible { } , Err (e) => Err (e) , } } # [inline] fn writeable_length_hint (& self) -> LengthHint { self . 0 . writeable_length_hint () } # [inline] # [cfg (feature = "alloc")] fn write_to_string (& self) -> Cow < '_ , str > { match self . 0 . try_write_to_string () { Ok (s) => s , Err ((infallible , _)) => match infallible { } , } } }
};
}
