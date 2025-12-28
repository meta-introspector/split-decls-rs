use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn derive_try_from_bytes_inner (ast : & DeriveInput , top_level : Trait , zerocopy_crate : & Path ,) -> Result < TokenStream , Error > { match & ast . data { Data :: Struct (strct) => derive_try_from_bytes_struct (ast , strct , top_level , zerocopy_crate) , Data :: Enum (enm) => derive_try_from_bytes_enum (ast , enm , top_level , zerocopy_crate) , Data :: Union (unn) => Ok (derive_try_from_bytes_union (ast , unn , top_level , zerocopy_crate ,)) , } }
}