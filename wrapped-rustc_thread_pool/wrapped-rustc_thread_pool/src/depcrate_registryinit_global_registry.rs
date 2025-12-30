// Generated macro for init_global_registry (function)
macro_rules! Depcrate_registryinit_global_registry {
() => {
// Module: crate::registry
// Provides: {"init_global_registry"}
// Dependencies: {}
# [doc = " Starts the worker threads (if that has not already happened) with"] # [doc = " the given builder."] pub (super) fn init_global_registry < S > (builder : ThreadPoolBuilder < S > ,) -> Result < & 'static Arc < Registry > , ThreadPoolBuildError > where S : ThreadSpawn , { set_global_registry (| | Registry :: new (builder)) }
};
}
