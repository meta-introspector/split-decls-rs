use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A struct is `FromBytes` if:"] # [doc = " - all fields are `FromBytes`"] fn derive_from_bytes_struct (ast : & DeriveInput , strct : & DataStruct , zerocopy_crate : & Path ,) -> TokenStream { ImplBlockBuilder :: new (ast , strct , Trait :: FromBytes , FieldBounds :: ALL_SELF , zerocopy_crate ,) . build () }