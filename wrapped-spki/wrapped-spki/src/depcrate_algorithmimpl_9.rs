// Generated macro for impl_9 (impl)
macro_rules! Depcrate_algorithmimpl_9 {
() => {
// Module: crate::algorithm
// Provides: {"impl_9"}
// Dependencies: {}
impl < 'a , Params > DecodeValue < 'a > for AlgorithmIdentifier < Params > where Params : Choice < 'a , Error = der :: Error > , { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , _header : Header) -> der :: Result < Self > { Ok (Self { oid : reader . decode () ? , parameters : reader . decode () ? , }) } }
};
}
