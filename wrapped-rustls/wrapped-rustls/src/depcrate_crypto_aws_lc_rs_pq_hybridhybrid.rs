// Generated macro for Hybrid (struct)
macro_rules! Depcrate_crypto_aws_lc_rs_pq_hybridHybrid {
() => {
// Module: crate::crypto::aws_lc_rs::pq::hybrid
// Provides: {"Hybrid"}
// Dependencies: {}
# [doc = " A generalization of hybrid key exchange."] # [derive (Debug)] pub (crate) struct Hybrid { pub (crate) classical : & 'static dyn SupportedKxGroup , pub (crate) post_quantum : & 'static dyn SupportedKxGroup , pub (crate) name : NamedGroup , pub (crate) layout : Layout , }
};
}
