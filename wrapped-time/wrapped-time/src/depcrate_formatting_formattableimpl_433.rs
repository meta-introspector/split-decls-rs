// Generated macro for impl_433 (impl)
macro_rules! Depcrate_formatting_formattableimpl_433 {
() => {
// Module: crate::formatting::formattable
// Provides: {"impl_433"}
// Dependencies: {}
impl sealed :: Sealed for OwnedFormatItem { # [inline] fn format_into (& self , output : & mut (impl io :: Write + ? Sized) , date : Option < Date > , time : Option < Time > , offset : Option < UtcOffset > ,) -> Result < usize , error :: Format > { match self { Self :: Literal (literal) => Ok (write (output , literal) ?) , Self :: Component (component) => format_component (output , * component , date , time , offset) , Self :: Compound (items) => items . format_into (output , date , time , offset) , Self :: Optional (item) => item . format_into (output , date , time , offset) , Self :: First (items) => match & * * items { [] => Ok (0) , [item , ..] => item . format_into (output , date , time , offset) , } , } } }
};
}
