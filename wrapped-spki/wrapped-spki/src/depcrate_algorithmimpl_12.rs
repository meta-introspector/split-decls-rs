// Generated macro for impl_12 (impl)
macro_rules! Depcrate_algorithmimpl_12 {
() => {
// Module: crate::algorithm
// Provides: {"impl_12"}
// Dependencies: {}
impl < 'a , Params > TryFrom < & 'a [u8] > for AlgorithmIdentifier < Params > where Params : Choice < 'a , Error = der :: Error > + Encode , { type Error = Error ; fn try_from (bytes : & 'a [u8]) -> Result < Self > { Ok (Self :: from_der (bytes) ?) } }
};
}
