// Generated macro for impl_261 (impl)
macro_rules! Depcrate_serdeimpl_261 {
() => {
// Module: crate::serde
// Provides: {"impl_261"}
// Dependencies: {}
# [cfg (feature = "system")] impl serde :: Serialize for crate :: System { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { let mut state = serializer . serialize_struct ("System" , 19) ? ; state . serialize_field ("global_cpu_usage" , & self . global_cpu_usage ()) ? ; state . serialize_field ("cpus" , & self . cpus ()) ? ; state . serialize_field ("physical_core_count" , & Self :: physical_core_count ()) ? ; state . serialize_field ("total_memory" , & self . total_memory ()) ? ; state . serialize_field ("free_memory" , & self . free_memory ()) ? ; state . serialize_field ("available_memory" , & self . available_memory ()) ? ; state . serialize_field ("used_memory" , & self . used_memory ()) ? ; state . serialize_field ("total_swap" , & self . total_swap ()) ? ; state . serialize_field ("free_swap" , & self . free_swap ()) ? ; state . serialize_field ("used_swap" , & self . used_swap ()) ? ; state . serialize_field ("uptime" , & Self :: uptime ()) ? ; state . serialize_field ("boot_time" , & Self :: boot_time ()) ? ; state . serialize_field ("load_average" , & Self :: load_average ()) ? ; state . serialize_field ("name" , & Self :: name ()) ? ; state . serialize_field ("kernel_version" , & Self :: kernel_version ()) ? ; state . serialize_field ("os_version" , & Self :: os_version ()) ? ; state . serialize_field ("long_os_version" , & Self :: long_os_version ()) ? ; state . serialize_field ("distribution_id" , & Self :: distribution_id ()) ? ; state . serialize_field ("host_name" , & Self :: host_name ()) ? ; state . end () } }
};
}
