// Generated macro for impl_30 (impl)
macro_rules! Depcrate_time_serdeimpl_30 {
() => {
// Module: crate::time::serde
// Provides: {"impl_30"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "serde")))] impl Serialize for SystemTime { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { use serde :: ser :: SerializeStruct ; let duration_since_epoch = self . 0 ; let mut state = serializer . serialize_struct ("SystemTime" , 2) ? ; state . serialize_field ("secs_since_epoch" , & duration_since_epoch . as_secs ()) ? ; state . serialize_field ("nanos_since_epoch" , & duration_since_epoch . subsec_nanos ()) ? ; state . end () } }
};
}
