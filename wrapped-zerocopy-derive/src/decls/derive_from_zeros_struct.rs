macro_rules! deps {
    () => {
        Trait!();
        FieldBounds!();
        ImplBlockBuilder!();
    };
}

macro_rules! derive_from_zeros_struct {
    () => {
        deps!();
        # [doc = " A struct is `FromZeros` if:"] # [doc = " - all fields are `FromZeros`"] fn derive_from_zeros_struct (ast : & DeriveInput , strct : & DataStruct , zerocopy_crate : & Path ,) -> TokenStream { ImplBlockBuilder :: new (ast , strct , Trait :: FromZeros , FieldBounds :: ALL_SELF , zerocopy_crate) . build () }
    };
}

derive_from_zeros_struct!()