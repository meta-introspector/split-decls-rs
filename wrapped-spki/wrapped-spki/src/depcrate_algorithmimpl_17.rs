// Generated macro for impl_17 (impl)
macro_rules! Depcrate_algorithmimpl_17 {
() => {
// Module: crate::algorithm
// Provides: {"impl_17"}
// Dependencies: {}
impl < Params > AlgorithmIdentifier < Params > { # [doc = " Assert the `algorithm` OID is an expected value."] pub fn assert_algorithm_oid (& self , expected_oid : ObjectIdentifier) -> Result < ObjectIdentifier > { if self . oid == expected_oid { Ok (expected_oid) } else { Err (Error :: OidUnknown { oid : self . oid }) } } }
};
}
