// Generated macro for impl_1956 (impl)
macro_rules! Depcrate_suitesimpl_1956 {
() => {
// Module: crate::suites
// Provides: {"impl_1956"}
// Dependencies: {}
impl SupportedCipherSuite { # [doc = " The cipher suite's identifier"] pub fn suite (& self) -> CipherSuite { self . common () . suite } # [doc = " The hash function the ciphersuite uses."] pub (crate) fn hash_provider (& self) -> & 'static dyn crypto :: hash :: Hash { self . common () . hash_provider } pub (crate) fn common (& self) -> & CipherSuiteCommon { match self { Self :: Tls12 (inner) => & inner . common , Self :: Tls13 (inner) => & inner . common , } } # [doc = " Return true if this suite is usable for the given [`Protocol`]."] pub (crate) fn usable_for_protocol (& self , proto : Protocol) -> bool { match self { Self :: Tls12 (tls12) => tls12 . usable_for_protocol (proto) , Self :: Tls13 (tls13) => tls13 . usable_for_protocol (proto) , } } }
};
}
