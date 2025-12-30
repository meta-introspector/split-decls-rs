// Generated macro for impl_101 (impl)
macro_rules! Depcrate_quic_vecimpl_101 {
() => {
// Module: crate::quic_vec
// Provides: {"impl_101"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < 'a > Arbitrary < 'a > for VLBytes { fn arbitrary (u : & mut Unstructured < 'a >) -> arbitrary :: Result < Self > { let mut vec = Vec :: arbitrary (u) ? ; vec . truncate (MAX_LEN as usize) ; Ok (Self { vec }) } }
};
}
