// Generated macro for impl_63 (impl)
macro_rules! Depcrateimpl_63 {
() => {
// Module: crate
// Provides: {"impl_63"}
// Dependencies: {}
impl crypto :: SigningKey for SigningKey { fn choose_scheme (& self , offered : & [SignatureScheme]) -> Option < Box < dyn crypto :: Signer > > { match offered . contains (& SIGNATURE_SCHEME) { true => Some (Box :: new (Self)) , false => None , } } fn public_key (& self) -> Option < SubjectPublicKeyInfoDer < '_ > > { None } }
};
}
