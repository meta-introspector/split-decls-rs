// Generated macro for check_struct (macro)
macro_rules! Depcrate_check_typescheck_struct {
() => {
// Module: crate::check_types
// Provides: {"check_struct"}
// Dependencies: {}
# [doc = " For the common case of no renaming, check all fields of a struct."] macro_rules ! check_struct { ($ name : ident , $ ($ field : ident) ,*) => { check_type ! ($ name) ; if false { # [allow (unreachable_code)] let _test = $ name { $ ($ field : panic ! ()) ,* } ; # [allow (unreachable_code)] let _test = c ::$ name { $ ($ field : panic ! ()) ,* } ; } $ (check_struct_field ! ($ name , $ field)) ;* } ; }
};
}
