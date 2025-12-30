// Generated macro for IssuingDistributionPoint (struct)
macro_rules! Depcrate_crl_typesIssuingDistributionPoint {
() => {
// Module: crate::crl::types
// Provides: {"IssuingDistributionPoint"}
// Dependencies: {}
pub (crate) struct IssuingDistributionPoint < 'a > { distribution_point : Option < untrusted :: Input < 'a > > , pub (crate) only_contains_user_certs : bool , pub (crate) only_contains_ca_certs : bool , pub (crate) only_some_reasons : Option < der :: BitStringFlags < 'a > > , pub (crate) indirect_crl : bool , pub (crate) only_contains_attribute_certs : bool , }
};
}
