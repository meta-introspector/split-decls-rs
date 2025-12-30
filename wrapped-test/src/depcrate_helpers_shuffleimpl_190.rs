// Generated macro for impl_190 (impl)
macro_rules! Depcrate_helpers_shuffleimpl_190 {
() => {
// Module: crate::helpers::shuffle
// Provides: {"impl_190"}
// Dependencies: {}
impl Rng { fn new (seed : u64 , extra : u64) -> Self { Self { state : seed , extra } } fn rand_range (& mut self , range : core :: ops :: Range < u64 >) -> u64 { self . rand_u64 () % (range . end - range . start) + range . start } fn rand_u64 (& mut self) -> u64 { self . state = calculate_hash (& (self . state , self . extra)) ; self . state } }
};
}
