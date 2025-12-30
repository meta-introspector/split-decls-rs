// Generated macro for impl_255 (impl)
macro_rules! Depcrate_serdeimpl_255 {
() => {
// Module: crate::serde
// Provides: {"impl_255"}
// Dependencies: {}
# [cfg (feature = "disk")] impl Serialize for crate :: Disk { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = serializer . serialize_struct ("Disk" , 7) ? ; state . serialize_field ("DiskKind" , & self . kind ()) ? ; if let Some (s) = self . name () . to_str () { state . serialize_field ("name" , s) ? ; } state . serialize_field ("file_system" , & self . file_system ()) ? ; state . serialize_field ("mount_point" , & self . mount_point ()) ? ; state . serialize_field ("total_space" , & self . total_space ()) ? ; state . serialize_field ("available_space" , & self . available_space ()) ? ; state . serialize_field ("is_removable" , & self . is_removable ()) ? ; state . end () } }
};
}
