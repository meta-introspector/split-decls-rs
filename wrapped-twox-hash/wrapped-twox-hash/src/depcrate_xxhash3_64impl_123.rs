// Generated macro for impl_123 (impl)
macro_rules! Depcrate_xxhash3_64impl_123 {
() => {
// Module: crate::xxhash3_64
// Provides: {"impl_123"}
// Dependencies: {}
impl Finalize for Finalize64 { type Output = u64 ; # [inline (always)] fn small (& self , secret : & Secret , seed : u64 , input : & [u8]) -> Self :: Output { impl_oneshot (secret , seed , input) } # [inline (always)] fn large (& self , vector : impl Vector , acc : [u64 ; 8] , last_block : & [u8] , last_stripe : & [u8 ; 64] , secret : & Secret , len : usize ,) -> Self :: Output { Algorithm (vector) . finalize_64 (acc , last_block , last_stripe , secret , len) } }
};
}
