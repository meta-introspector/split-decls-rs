// Generated macro for impl_82 (impl)
macro_rules! Depcrate_cycleimpl_82 {
() => {
// Module: crate::cycle
// Provides: {"impl_82"}
// Dependencies: {}
impl Clone for CycleHead { fn clone (& self) -> Self { Self { database_key_index : self . database_key_index , iteration_count : self . iteration_count . load () . into () , removed : self . removed . load (Ordering :: Relaxed) . into () , } } }
};
}
