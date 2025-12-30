// Generated macro for sealed (module)
macro_rules! Depcrate_parsing_parsedsealed {
() => {
// Module: crate::parsing::parsed
// Provides: {"sealed"}
// Dependencies: {}
# [doc = " Sealed to prevent downstream implementations."] mod sealed { use super :: * ; # [doc = " A trait to allow `parse_item` to be generic."] pub trait AnyFormatItem { # [doc = " Parse a single item, returning the remaining input on success."] fn parse_item < 'a > (& self , parsed : & mut Parsed , input : & 'a [u8] ,) -> Result < & 'a [u8] , error :: ParseFromDescription > ; } }
};
}
