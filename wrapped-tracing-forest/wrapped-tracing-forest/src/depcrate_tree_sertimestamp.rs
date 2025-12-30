// Generated macro for timestamp (function)
macro_rules! Depcrate_tree_sertimestamp {
() => {
// Module: crate::tree::ser
// Provides: {"timestamp"}
// Dependencies: {}
# [cfg (feature = "chrono")] pub (super) fn timestamp < S : Serializer > (timestamp : & DateTime < Utc > , serializer : S ,) -> Result < S :: Ok , S :: Error > { serializer . serialize_str (& timestamp . to_rfc3339 ()) }
};
}
