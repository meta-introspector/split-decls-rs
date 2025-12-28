macro_rules! deps {
    () => {
        ImplBlockBuilder!();
        Trait!();
        FieldBounds!();
        EnumRepr!();
        Repr!();
        CompoundRepr!();
    };
}

macro_rules! derive_from_zeros_enum {
    () => {
        deps!();
        # [doc = " An enum is `FromZeros` if:"] # [doc = " - one of the variants has a discriminant of `0`"] # [doc = " - that variant's fields are all `FromZeros`"] fn derive_from_zeros_enum (ast : & DeriveInput , enm : & DataEnum , zerocopy_crate : & Path ,) -> Result < TokenStream , Error > { let repr = EnumRepr :: from_attrs (& ast . attrs) ? ; match repr { Repr :: Compound (Spanned { t : CompoundRepr :: C | CompoundRepr :: Primitive (_) , span : _ } , _ ,) => { } Repr :: Transparent (_) | Repr :: Compound (Spanned { t : CompoundRepr :: Rust , span : _ } , _) => return Err (Error :: new (Span :: call_site () , "must have #[repr(C)] or #[repr(Int)] attribute in order to guarantee this type's memory layout")) , } let zero_variant = match find_zero_variant (enm) { Ok (index) => enm . variants . iter () . nth (index) . unwrap () , Err (true) => { return Err (Error :: new_spanned (ast , "FromZeros only supported on enums with a variant that has a discriminant of `0`\n\
                help: This enum has discriminants which are not literal integers. One of those may \
                define or imply which variant has a discriminant of zero. Use a literal integer to \
                define or imply the variant with a discriminant of zero." ,)) ; } Err (false) => { return Err (Error :: new_spanned (ast , "FromZeros only supported on enums with a variant that has a discriminant of `0`" ,)) ; } } ; let explicit_bounds = zero_variant . fields . iter () . map (| field | { let ty = & field . ty ; parse_quote ! { # ty : # zerocopy_crate :: FromZeros } }) . collect :: < Vec < WherePredicate > > () ; Ok (ImplBlockBuilder :: new (ast , enm , Trait :: FromZeros , FieldBounds :: Explicit (explicit_bounds) , zerocopy_crate ,) . build ()) }
    };
}

derive_from_zeros_enum!()