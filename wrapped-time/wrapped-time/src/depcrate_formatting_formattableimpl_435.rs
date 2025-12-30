// Generated macro for impl_435 (impl)
macro_rules! Depcrate_formatting_formattableimpl_435 {
() => {
// Module: crate::formatting::formattable
// Provides: {"impl_435"}
// Dependencies: {}
impl < T > sealed :: Sealed for T where T : Deref < Target : sealed :: Sealed > , { # [inline] fn format_into (& self , output : & mut (impl io :: Write + ? Sized) , date : Option < Date > , time : Option < Time > , offset : Option < UtcOffset > ,) -> Result < usize , error :: Format > { self . deref () . format_into (output , date , time , offset) } }
};
}
