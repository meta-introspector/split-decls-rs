macro_rules! check_struct_field {
    () => {
        # [doc = " Check that the field of a struct has the same offset as the corresponding"] # [doc = " field in the `sys` bindings."] macro_rules ! check_struct_field { ($ struct : ident , $ field : ident) => { const_assert_eq ! (memoffset :: offset_of ! ($ struct , $ field) , memoffset :: offset_of ! (c ::$ struct , $ field)) ; assert_eq ! (memoffset :: span_of ! ($ struct , $ field) , memoffset :: span_of ! (c ::$ struct , $ field)) ; } ; }
    };
}

check_struct_field!();