// Generated macro for impl_269 (impl)
macro_rules! Depcrate_serdeimpl_269 {
() => {
// Module: crate::serde
// Provides: {"impl_269"}
// Dependencies: {}
# [cfg (feature = "system")] impl Serialize for crate :: DiskUsage { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = serializer . serialize_struct ("DiskUsage" , 4) ? ; state . serialize_field ("total_written_bytes" , & self . total_written_bytes) ? ; state . serialize_field ("written_bytes" , & self . written_bytes) ? ; state . serialize_field ("total_read_bytes" , & self . total_read_bytes) ? ; state . serialize_field ("read_bytes" , & self . read_bytes) ? ; state . end () } }
};
}
