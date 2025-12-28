macro_rules! check_struct_renamed_field {
    () => {
        # [doc = " The same as `check_struct_field`, but for unions and anonymous structs"] # [doc = " we've renamed to avoid having types like `bindgen_ty_1` in the API."] macro_rules ! check_struct_renamed_field { ($ struct : ident , $ to : ident , $ from : ident) => { const_assert_eq ! (memoffset :: offset_of ! ($ struct , $ to) , memoffset :: offset_of ! (c ::$ struct , $ from)) ; assert_eq ! (memoffset :: span_of ! ($ struct , $ to) , memoffset :: span_of ! (c ::$ struct , $ from)) ; } ; }
    };
}

check_struct_renamed_field!();