// Generated macro for check_eku (function)
macro_rules! Depcrate_verify_certcheck_eku {
() => {
// Module: crate::verify_cert
// Provides: {"check_eku"}
// Dependencies: {}
fn check_eku (input : Option < & mut untrusted :: Reader < '_ > > , eku : & dyn ExtendedKeyUsageValidator ,) -> Result < () , Error > { match input { Some (input) if input . at_end () => Err (Error :: EmptyEkuExtension) , Some (input) => eku . validate (KeyPurposeIdIter { input }) , None => eku . validate (KeyPurposeIdIter { input : & mut untrusted :: Reader :: new (untrusted :: Input :: from (& [])) , }) , } }
};
}
