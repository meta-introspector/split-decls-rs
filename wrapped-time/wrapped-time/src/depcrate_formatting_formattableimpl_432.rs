// Generated macro for impl_432 (impl)
macro_rules! Depcrate_formatting_formattableimpl_432 {
() => {
// Module: crate::formatting::formattable
// Provides: {"impl_432"}
// Dependencies: {}
impl sealed :: Sealed for [BorrowedFormatItem < '_ >] { # [inline] fn format_into (& self , output : & mut (impl io :: Write + ? Sized) , date : Option < Date > , time : Option < Time > , offset : Option < UtcOffset > ,) -> Result < usize , error :: Format > { let mut bytes = 0 ; for item in self . iter () { bytes += item . format_into (output , date , time , offset) ? ; } Ok (bytes) } }
};
}
