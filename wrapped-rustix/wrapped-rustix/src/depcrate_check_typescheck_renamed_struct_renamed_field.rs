// Generated macro for check_renamed_struct_renamed_field (macro)
macro_rules! Depcrate_check_typescheck_renamed_struct_renamed_field {
() => {
// Module: crate::check_types
// Provides: {"check_renamed_struct_renamed_field"}
// Dependencies: {}
# [doc = " The same as `check_struct_renamed_field`, but for when both the struct and"] # [doc = " a field are renamed."] macro_rules ! check_renamed_struct_renamed_field { ($ to_struct : ident , $ from_struct : ident , $ to : ident , $ from : ident) => { const_assert_eq ! (memoffset :: offset_of ! ($ to_struct , $ to) , memoffset :: offset_of ! (c ::$ from_struct , $ from)) ; assert_eq ! (memoffset :: span_of ! ($ to_struct , $ to) , memoffset :: span_of ! (c ::$ from_struct , $ from)) ; } ; }
};
}
