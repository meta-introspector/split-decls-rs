// Generated macro for read_all (function)
macro_rules! Depcrate_derread_all {
() => {
// Module: crate::der
// Provides: {"read_all"}
// Dependencies: {}
pub (crate) fn read_all < 'a , T : FromDer < 'a > > (input : untrusted :: Input < 'a >) -> Result < T , Error > { input . read_all (Error :: TrailingData (T :: TYPE_ID) , T :: from_der) }
};
}
