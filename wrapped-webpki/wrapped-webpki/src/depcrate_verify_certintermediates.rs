// Generated macro for Intermediates (enum)
macro_rules! Depcrate_verify_certIntermediates {
() => {
// Module: crate::verify_cert
// Provides: {"Intermediates"}
// Dependencies: {}
# [allow (clippy :: large_enum_variant)] enum Intermediates < 'a > { Owned { certs : [Option < Cert < 'a > > ; MAX_SUB_CA_COUNT] , used : usize , } , Borrowed (& 'a [Option < Cert < 'a > >]) , }
};
}
