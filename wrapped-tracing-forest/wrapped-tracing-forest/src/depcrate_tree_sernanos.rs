// Generated macro for nanos (function)
macro_rules! Depcrate_tree_sernanos {
() => {
// Module: crate::tree::ser
// Provides: {"nanos"}
// Dependencies: {}
pub (super) fn nanos < S : Serializer > (duration : & Duration , serializer : S) -> Result < S :: Ok , S :: Error > { serializer . serialize_u128 (duration . as_nanos ()) }
};
}
