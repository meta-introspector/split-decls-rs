// Generated macro for get_hash_map_string_element (function)
macro_rules! Depcrate_struct_metaget_hash_map_string_element {
() => {
// Module: crate::struct_meta
// Provides: {"get_hash_map_string_element"}
// Dependencies: {}
fn get_hash_map_string_element (ty : & Type) -> Option < & Type > { let (ty_key , ty_value) = get_hash_map_element (ty) ? ; if is_string (ty_key) { Some (ty_value) } else { None } }
};
}
