// Generated macro for impl_162 (impl)
macro_rules! Depcrate_signed_dataimpl_162 {
() => {
// Module: crate::signed_data
// Provides: {"impl_162"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl OwnedSignedData { # [doc = " Return a borrowed [`SignedData`] from the owned representation."] pub (crate) fn borrow (& self) -> SignedData < '_ > { SignedData { data : untrusted :: Input :: from (& self . data) , algorithm : untrusted :: Input :: from (& self . algorithm) , signature : untrusted :: Input :: from (& self . signature) , } } }
};
}
