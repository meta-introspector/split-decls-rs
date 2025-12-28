macro_rules! deps {
    () => {
        KnownLayout!();
        DstLayout!();
    };
}

macro_rules! repr_c_struct_has_padding {
    () => {
        deps!();
        # [doc = " Does the `repr(C)` struct type `$t` have padding?"] # [doc = ""] # [doc = " `$ts` is the list of the type of every field in `$t`. `$t` must be a"] # [doc = " `repr(C)` struct type, or else `struct_has_padding!`'s result may be"] # [doc = " meaningless."] # [doc (hidden)] # [macro_export] macro_rules ! repr_c_struct_has_padding { ($ t : ty , [$ ($ ts : tt) ,*]) => { { let layout = $ crate :: DstLayout :: for_repr_c_struct ($ crate :: util :: macro_util :: core_reexport :: option :: Option :: None , $ crate :: util :: macro_util :: core_reexport :: option :: Option :: None , & [$ ($ crate :: repr_c_struct_has_padding ! (@ field $ ts) ,) *]) ; layout . requires_static_padding () || layout . requires_dynamic_padding () } } ; (@ field [$ t : ty]) => { < [$ t] as $ crate :: KnownLayout >:: LAYOUT } ; (@ field $ t : ty) => { $ crate :: DstLayout :: for_unpadded_type ::<$ t > () } ; }
    };
}

repr_c_struct_has_padding!()