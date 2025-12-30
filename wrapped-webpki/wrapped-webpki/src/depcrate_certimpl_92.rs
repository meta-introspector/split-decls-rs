// Generated macro for impl_92 (impl)
macro_rules! Depcrate_certimpl_92 {
() => {
// Module: crate::cert
// Provides: {"impl_92"}
// Dependencies: {}
impl < 'a > CrlDistributionPoint < 'a > { # [doc = " Return the distribution point names (if any)."] pub (crate) fn names (& self) -> Result < Option < DistributionPointName < 'a > > , Error > { self . distribution_point . map (| input | DistributionPointName :: from_der (& mut untrusted :: Reader :: new (input))) . transpose () } }
};
}
