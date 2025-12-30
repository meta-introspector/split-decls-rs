// Generated macro for impl_81 (impl)
macro_rules! Depcrate_cycleimpl_81 {
() => {
// Module: crate::cycle
// Provides: {"impl_81"}
// Dependencies: {}
impl CycleHead { pub const fn new (database_key_index : DatabaseKeyIndex , iteration_count : IterationCount ,) -> Self { Self { database_key_index , iteration_count : AtomicIterationCount (AtomicU8 :: new (iteration_count . 0)) , removed : AtomicBool :: new (false) , } } }
};
}
