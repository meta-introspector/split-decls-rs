// Generated macro for check_renamed_struct_field (macro)
macro_rules! Depcrate_check_typescheck_renamed_struct_field {
() => {
// Module: crate::check_types
// Provides: {"check_renamed_struct_field"}
// Dependencies: {}
# [doc = " The same as `check_struct_field`, but for when the struct is renamed"] # [doc = " but the field is not."] macro_rules ! check_renamed_struct_field { ($ to_struct : ident , $ from_struct : ident , $ field : ident) => { const_assert_eq ! (memoffset :: offset_of ! ($ to_struct , $ field) , memoffset :: offset_of ! (c ::$ from_struct , $ field)) ; assert_eq ! (memoffset :: span_of ! ($ to_struct , $ field) , memoffset :: span_of ! (c ::$ from_struct , $ field)) ; } ; }
};
}
