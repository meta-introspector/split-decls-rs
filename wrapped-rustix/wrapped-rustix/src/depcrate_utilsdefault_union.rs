// Generated macro for default_union (macro)
macro_rules! Depcrate_utilsdefault_union {
() => {
// Module: crate::utils
// Provides: {"default_union"}
// Dependencies: {}
# [doc = " Create a union value containing a default value in one of its arms."] # [doc = ""] # [doc = " The field names a union field which must have the same size as the union"] # [doc = " itself."] macro_rules ! default_union { ($ union : ident , $ field : ident) => { { let u = $ union { $ field : Default :: default () , } ; # [cfg (test)] unsafe { let field_value = u .$ field ; assert_eq ! (core :: mem :: size_of_val (& u) , core :: mem :: size_of_val (& field_value)) ; const_assert_eq ! (memoffset :: offset_of_union ! ($ union , $ field) , 0) ; } u } } ; }
};
}
