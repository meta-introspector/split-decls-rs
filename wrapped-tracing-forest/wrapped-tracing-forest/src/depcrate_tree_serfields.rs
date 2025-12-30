// Generated macro for fields (function)
macro_rules! Depcrate_tree_serfields {
() => {
// Module: crate::tree::ser
// Provides: {"fields"}
// Dependencies: {}
pub (super) fn fields < S : Serializer > (fields : & FieldSet , serializer : S) -> Result < S :: Ok , S :: Error > { let mut model = serializer . serialize_map (Some (fields . len ())) ? ; for field in fields { model . serialize_entry (field . key () , field . value ()) ? ; } model . end () }
};
}
