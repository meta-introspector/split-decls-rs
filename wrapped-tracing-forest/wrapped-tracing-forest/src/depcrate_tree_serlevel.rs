// Generated macro for level (function)
macro_rules! Depcrate_tree_serlevel {
() => {
// Module: crate::tree::ser
// Provides: {"level"}
// Dependencies: {}
# [allow (clippy :: trivially_copy_pass_by_ref)] pub (super) fn level < S : Serializer > (level : & Level , serializer : S) -> Result < S :: Ok , S :: Error > { serializer . serialize_str (level . as_str ()) }
};
}
