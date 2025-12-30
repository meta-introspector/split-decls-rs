// Generated macro for pax_extensions_value (function)
macro_rules! Depcrate_paxpax_extensions_value {
() => {
// Module: crate::pax
// Provides: {"pax_extensions_value"}
// Dependencies: {}
pub fn pax_extensions_value (a : & [u8] , key : & str) -> Option < u64 > { for extension in PaxExtensions :: new (a) { let current_extension = match extension { Ok (ext) => ext , Err (_) => return None , } ; if current_extension . key () != Ok (key) { continue ; } let value = match current_extension . value () { Ok (value) => value , Err (_) => return None , } ; let result = match value . parse :: < u64 > () { Ok (result) => result , Err (_) => return None , } ; return Some (result) ; } None }
};
}
