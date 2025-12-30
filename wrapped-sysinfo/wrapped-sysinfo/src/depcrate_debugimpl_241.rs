// Generated macro for impl_241 (impl)
macro_rules! Depcrate_debugimpl_241 {
() => {
// Module: crate::debug
// Provides: {"impl_241"}
// Dependencies: {}
# [cfg (feature = "system")] impl std :: fmt :: Debug for crate :: System { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("System") . field ("global CPU usage" , & self . global_cpu_usage ()) . field ("load average" , & Self :: load_average ()) . field ("total memory" , & self . total_memory ()) . field ("free memory" , & self . free_memory ()) . field ("total swap" , & self . total_swap ()) . field ("free swap" , & self . free_swap ()) . field ("nb CPUs" , & self . cpus () . len ()) . field ("nb processes" , & self . processes () . len ()) . finish () } }
};
}
