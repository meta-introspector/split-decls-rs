// Generated macro for check_renamed_struct (macro)
macro_rules! Depcrate_check_typescheck_renamed_struct {
() => {
// Module: crate::check_types
// Provides: {"check_renamed_struct"}
// Dependencies: {}
# [doc = " For the case of renaming, check all fields of a struct."] macro_rules ! check_renamed_struct { ($ to_struct : ident , $ from_struct : ident , $ ($ field : ident) ,*) => { check_renamed_type ! ($ to_struct , $ from_struct) ; if false { # [allow (unreachable_code)] let _test = $ to_struct { $ ($ field : panic ! ()) ,* } ; # [allow (unreachable_code)] let _test = c ::$ from_struct { $ ($ field : panic ! ()) ,* } ; } $ (check_renamed_struct_field ! ($ to_struct , $ from_struct , $ field)) ;* } ; }
};
}
