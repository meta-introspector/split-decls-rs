// Generated macro for impl_81 (impl)
macro_rules! Depcrate_parallel_hashimpl_81 {
() => {
// Module: crate::parallel_hash
// Provides: {"impl_81"}
// Dependencies: {}
impl ParallelHash { # [doc = " Creates  new [`ParallelHash`] hasher with a security level of 128 bits."] # [doc = ""] # [doc = " [`ParallelHash`]: struct.ParallelHash.html"] pub fn v128 (custom_string : & [u8] , block_size : usize) -> ParallelHash { ParallelHash :: new (custom_string , block_size , 128) } # [doc = " Creates  new [`ParallelHash`] hasher with a security level of 256 bits."] # [doc = ""] # [doc = " [`ParallelHash`]: struct.ParallelHash.html"] pub fn v256 (custom_string : & [u8] , block_size : usize) -> ParallelHash { ParallelHash :: new (custom_string , block_size , 256) } fn new (custom_string : & [u8] , block_size : usize , bits : usize) -> ParallelHash { let mut state = CShake :: new (b"ParallelHash" , custom_string , bits) ; state . update (left_encode (block_size) . value ()) ; ParallelHash { state , block_size , bits , blocks : 0 , unfinished : None , } } }
};
}
