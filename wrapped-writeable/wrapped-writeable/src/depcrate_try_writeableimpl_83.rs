// Generated macro for impl_83 (impl)
macro_rules! Depcrate_try_writeableimpl_83 {
() => {
// Module: crate::try_writeable
// Provides: {"impl_83"}
// Dependencies: {}
impl < T , E > TryWriteable for Result < T , E > where T : Writeable , E : Writeable + Clone , { type Error = E ; # [inline] fn try_write_to < W : fmt :: Write + ? Sized > (& self , sink : & mut W ,) -> Result < Result < () , Self :: Error > , fmt :: Error > { match self { Ok (t) => t . write_to (sink) . map (Ok) , Err (e) => e . write_to (sink) . map (| () | Err (e . clone ())) , } } # [inline] fn try_write_to_parts < S : PartsWrite + ? Sized > (& self , sink : & mut S ,) -> Result < Result < () , Self :: Error > , fmt :: Error > { match self { Ok (t) => t . write_to_parts (sink) . map (Ok) , Err (e) => sink . with_part (Part :: ERROR , | sink | e . write_to_parts (sink)) . map (| () | Err (e . clone ())) , } } # [inline] fn writeable_length_hint (& self) -> LengthHint { match self { Ok (t) => t . writeable_length_hint () , Err (e) => e . writeable_length_hint () , } } # [inline] # [cfg (feature = "alloc")] fn try_write_to_string (& self) -> Result < Cow < '_ , str > , (Self :: Error , Cow < '_ , str >) > { match self { Ok (t) => Ok (t . write_to_string ()) , Err (e) => Err ((e . clone () , e . write_to_string ())) , } } }
};
}
