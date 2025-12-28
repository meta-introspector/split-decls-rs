macro_rules! deps {
    () => {
        Trait!();
        ImplBlockBuilder!();
        FieldBounds!();
    };
}

macro_rules! derive_from_bytes_struct {
    () => {
        deps!();
        # [doc = " A struct is `FromBytes` if:"] # [doc = " - all fields are `FromBytes`"] fn derive_from_bytes_struct (ast : & DeriveInput , strct : & DataStruct , zerocopy_crate : & Path ,) -> TokenStream { ImplBlockBuilder :: new (ast , strct , Trait :: FromBytes , FieldBounds :: ALL_SELF , zerocopy_crate) . build () }
    };
}

derive_from_bytes_struct!();