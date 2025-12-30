// Generated macro for impl_274 (impl)
macro_rules! Depcrate_crl_typesimpl_274 {
() => {
// Module: crate::crl::types
// Provides: {"impl_274"}
// Dependencies: {}
impl core :: hash :: Hash for BorrowedCertRevocationList < '_ > { fn hash < H : core :: hash :: Hasher > (& self , state : & mut H) { let Self { signed_data , issuer , issuing_distribution_point , revoked_certs , next_update , } = self ; signed_data . hash (state) ; issuer . as_slice_less_safe () . hash (state) ; issuing_distribution_point . map (| i | i . as_slice_less_safe ()) . hash (state) ; revoked_certs . as_slice_less_safe () . hash (state) ; next_update . hash (state) ; } }
};
}
