// Generated macro for prerelease_identifier (function)
macro_rules! Depcrate_parseprerelease_identifier {
() => {
// Module: crate::parse
// Provides: {"prerelease_identifier"}
// Dependencies: {}
fn prerelease_identifier (input : & str) -> Result < (Prerelease , & str) , Error > { let (string , rest) = identifier (input , Position :: Pre) ? ; let identifier = unsafe { Identifier :: new_unchecked (string) } ; Ok ((Prerelease { identifier } , rest)) }
};
}
