// Generated macro for impl_44 (impl)
macro_rules! Depcrate_derimpl_44 {
() => {
// Module: crate::der
// Provides: {"impl_44"}
// Dependencies: {}
impl < 'a > FromDer < 'a > for bool { fn from_der (reader : & mut untrusted :: Reader < 'a >) -> Result < Self , Error > { if ! reader . peek (Tag :: Boolean . into ()) { return Ok (false) ; } nested (reader , Tag :: Boolean , Error :: TrailingData (Self :: TYPE_ID) , | input | match input . read_byte () { Ok (0xff) => Ok (true) , Ok (0x00) => Ok (false) , _ => Err (Error :: BadDer) , } ,) } const TYPE_ID : DerTypeId = DerTypeId :: Bool ; }
};
}
