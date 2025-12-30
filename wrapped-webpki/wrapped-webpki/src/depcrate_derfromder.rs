// Generated macro for FromDer (trait)
macro_rules! Depcrate_derFromDer {
() => {
// Module: crate::der
// Provides: {"FromDer"}
// Dependencies: {}
pub (crate) trait FromDer < 'a > : Sized + 'a { # [doc = " Parse a value of type `Self` from the given DER-encoded input."] fn from_der (reader : & mut untrusted :: Reader < 'a >) -> Result < Self , Error > ; const TYPE_ID : DerTypeId ; }
};
}
