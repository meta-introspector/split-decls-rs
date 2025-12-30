// Generated macro for impl_145 (impl)
macro_rules! Depcrate_xxhash3_128impl_145 {
() => {
// Module: crate::xxhash3_128
// Provides: {"impl_145"}
// Dependencies: {}
impl Finalize for Finalize128 { type Output = u128 ; # [inline] fn small (& self , secret : & Secret , seed : u64 , input : & [u8]) -> Self :: Output { impl_oneshot (secret , seed , input) } # [inline] fn large (& self , vector : impl Vector , acc : [u64 ; 8] , last_block : & [u8] , last_stripe : & [u8 ; 64] , secret : & Secret , len : usize ,) -> Self :: Output { Algorithm (vector) . finalize_128 (acc , last_block , last_stripe , secret , len) } }
};
}
