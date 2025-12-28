macro_rules! deps {
    () => {
        Trait!();
        ImplBlockBuilder!();
        FieldBounds!();
    };
}

macro_rules! derive_unaligned_union {
    () => {
        deps!();
        # [doc = " Like structs, a union is `Unaligned` if:"] # [doc = " - `repr(align)` is no more than 1 and either"] # [doc = "   - `repr(C)` or `repr(transparent)` and"] # [doc = "     - all fields `Unaligned`"] # [doc = "   - `repr(packed)`"] fn derive_unaligned_union (ast : & DeriveInput , unn : & DataUnion , zerocopy_crate : & Path ,) -> Result < TokenStream , Error > { let repr = StructUnionRepr :: from_attrs (& ast . attrs) ? ; repr . unaligned_validate_no_align_gt_1 () ? ; let field_type_trait_bounds = if repr . is_packed_1 () { FieldBounds :: None } else if repr . is_c () || repr . is_transparent () { FieldBounds :: ALL_SELF } else { return Err (Error :: new (Span :: call_site () , "must have #[repr(C)], #[repr(transparent)], or #[repr(packed)] attribute in order to guarantee this type's alignment")) ; } ; Ok (ImplBlockBuilder :: new (ast , unn , Trait :: Unaligned , field_type_trait_bounds , zerocopy_crate) . build ()) }
    };
}

derive_unaligned_union!()