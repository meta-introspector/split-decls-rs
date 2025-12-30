// Generated macro for impl_244 (impl)
macro_rules! Depcrate_debugimpl_244 {
() => {
// Module: crate::debug
// Provides: {"impl_244"}
// Dependencies: {}
# [cfg (feature = "system")] impl std :: fmt :: Debug for crate :: Process { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Process") . field ("pid" , & self . pid ()) . field ("parent" , & self . parent ()) . field ("name" , & self . name ()) . field ("environ" , & self . environ ()) . field ("command" , & self . cmd ()) . field ("executable path" , & self . exe ()) . field ("current working directory" , & self . cwd ()) . field ("memory usage" , & self . memory ()) . field ("virtual memory usage" , & self . virtual_memory ()) . field ("CPU usage" , & self . cpu_usage ()) . field ("accumulated CPU time" , & self . accumulated_cpu_time ()) . field ("status" , & self . status ()) . field ("root" , & self . root ()) . field ("disk_usage" , & self . disk_usage ()) . field ("user_id" , & self . user_id ()) . field ("effective_user_id" , & self . effective_user_id ()) . finish () } }
};
}
