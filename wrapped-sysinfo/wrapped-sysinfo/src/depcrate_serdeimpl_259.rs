// Generated macro for impl_259 (impl)
macro_rules! Depcrate_serdeimpl_259 {
() => {
// Module: crate::serde
// Provides: {"impl_259"}
// Dependencies: {}
# [cfg (feature = "system")] impl Serialize for crate :: Process { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = serializer . serialize_struct ("Process" , 19) ? ; state . serialize_field ("name" , & self . name () . to_string_lossy ()) ? ; state . serialize_field ("cmd" , & self . cmd ()) ? ; state . serialize_field ("exe" , & self . exe ()) ? ; state . serialize_field ("pid" , & self . pid () . as_u32 ()) ? ; state . serialize_field ("environ" , & self . environ ()) ? ; state . serialize_field ("cwd" , & self . cwd ()) ? ; state . serialize_field ("root" , & self . root ()) ? ; state . serialize_field ("memory" , & self . memory ()) ? ; state . serialize_field ("virtual_memory" , & self . virtual_memory ()) ? ; state . serialize_field ("parent" , & self . parent ()) ? ; state . serialize_field ("status" , & self . status ()) ? ; state . serialize_field ("start_time" , & self . start_time ()) ? ; state . serialize_field ("run_time" , & self . run_time ()) ? ; state . serialize_field ("cpu_usage" , & self . cpu_usage ()) ? ; state . serialize_field ("accumulated_cpu_time" , & self . accumulated_cpu_time ()) ? ; state . serialize_field ("disk_usage" , & self . disk_usage ()) ? ; state . serialize_field ("user_id" , & self . user_id ()) ? ; state . serialize_field ("group_id" , & self . group_id ()) ? ; state . serialize_field ("session_id" , & self . session_id ()) ? ; state . end () } }
};
}
