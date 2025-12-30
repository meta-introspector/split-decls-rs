// Generated macro for ActiveHybrid (struct)
macro_rules! Depcrate_crypto_aws_lc_rs_pq_hybridActiveHybrid {
() => {
// Module: crate::crypto::aws_lc_rs::pq::hybrid
// Provides: {"ActiveHybrid"}
// Dependencies: {}
struct ActiveHybrid { classical : Box < dyn ActiveKeyExchange > , post_quantum : Box < dyn ActiveKeyExchange > , name : NamedGroup , layout : Layout , combined_pub_key : Vec < u8 > , }
};
}
