// Generated macro for serialize_patch (function)
macro_rules! Depcrate_manifestserialize_patch {
() => {
// Module: crate::manifest
// Provides: {"serialize_patch"}
// Dependencies: {}
fn serialize_patch < S > (patch : & Map < String , RegistryPatch > , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut map = serializer . serialize_map (None) ? ; for (registry , patch) in patch { if ! patch . crates . is_empty () { map . serialize_entry (registry , patch) ? ; } } map . end () }
};
}
