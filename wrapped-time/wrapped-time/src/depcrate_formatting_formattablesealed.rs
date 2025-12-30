// Generated macro for sealed (module)
macro_rules! Depcrate_formatting_formattablesealed {
() => {
// Module: crate::formatting::formattable
// Provides: {"sealed"}
// Dependencies: {}
# [doc = " Seal the trait to prevent downstream users from implementing it."] mod sealed { use super :: * ; # [doc = " Format the item using a format description, the intended output, and the various components."] pub trait Sealed { # [doc = " Format the item into the provided output, returning the number of bytes written."] fn format_into (& self , output : & mut (impl io :: Write + ? Sized) , date : Option < Date > , time : Option < Time > , offset : Option < UtcOffset > ,) -> Result < usize , error :: Format > ; # [doc = " Format the item directly to a `String`."] # [inline] fn format (& self , date : Option < Date > , time : Option < Time > , offset : Option < UtcOffset > ,) -> Result < String , error :: Format > { let mut buf = Vec :: new () ; self . format_into (& mut buf , date , time , offset) ? ; Ok (String :: from_utf8_lossy (& buf) . into_owned ()) } } }
};
}
