// Generated macro for CollectorRoot (struct)
macro_rules! Depcrate_collectorCollectorRoot {
() => {
// Module: crate::collector
// Provides: {"CollectorRoot"}
// Dependencies: {}
# [doc = " Data stored in a [`CollectorRoot`] is shared among [`Collector`] instances."] # [derive (Debug , Default)] pub (super) struct CollectorRoot { epoch : AtomicU8 , chain_head : AtomicPtr < Collector > , }
};
}
