// Generated macro for impl_145 (impl)
macro_rules! Depcrate_schannel_credimpl_145 {
() => {
// Module: crate::schannel_cred
// Provides: {"impl_145"}
// Dependencies: {}
impl Protocol { fn dword (self , direction : Direction) -> u32 { match (self , direction) { (Protocol :: Ssl3 , Direction :: Inbound) => Identity :: SP_PROT_SSL3_SERVER , (Protocol :: Tls10 , Direction :: Inbound) => Identity :: SP_PROT_TLS1_0_SERVER , (Protocol :: Tls11 , Direction :: Inbound) => Identity :: SP_PROT_TLS1_1_SERVER , (Protocol :: Tls12 , Direction :: Inbound) => Identity :: SP_PROT_TLS1_2_SERVER , (Protocol :: Tls13 , Direction :: Inbound) => Identity :: SP_PROT_TLS1_3_SERVER , (Protocol :: Ssl3 , Direction :: Outbound) => Identity :: SP_PROT_SSL3_CLIENT , (Protocol :: Tls10 , Direction :: Outbound) => Identity :: SP_PROT_TLS1_0_CLIENT , (Protocol :: Tls11 , Direction :: Outbound) => Identity :: SP_PROT_TLS1_1_CLIENT , (Protocol :: Tls12 , Direction :: Outbound) => Identity :: SP_PROT_TLS1_2_CLIENT , (Protocol :: Tls13 , Direction :: Outbound) => Identity :: SP_PROT_TLS1_3_CLIENT , } } }
};
}
