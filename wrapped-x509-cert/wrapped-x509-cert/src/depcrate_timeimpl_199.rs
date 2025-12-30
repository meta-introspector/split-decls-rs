// Generated macro for impl_199 (impl)
macro_rules! Depcrate_timeimpl_199 {
() => {
// Module: crate::time
// Provides: {"impl_199"}
// Dependencies: {}
impl < 'a , P : Profile > DecodeValue < 'a > for Validity < P > { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , _header : Header) -> der :: Result < Self > { let not_before = reader . decode () ? ; let not_after = reader . decode () ? ; let out = Self { not_before , not_after , _profile : PhantomData , } ; Ok (out) } }
};
}
