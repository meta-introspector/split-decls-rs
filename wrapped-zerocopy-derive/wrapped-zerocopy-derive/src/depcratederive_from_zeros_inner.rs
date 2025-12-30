// Generated macro for derive_from_zeros_inner (function)
macro_rules! Depcratederive_from_zeros_inner {
() => {
// Module: crate
// Provides: {"derive_from_zeros_inner"}
// Dependencies: {}
fn derive_from_zeros_inner (ast : & DeriveInput , top_level : Trait , zerocopy_crate : & Path ,) -> Result < TokenStream , Error > { let try_from_bytes = derive_try_from_bytes_inner (ast , top_level , zerocopy_crate) ? ; let from_zeros = match & ast . data { Data :: Struct (strct) => derive_from_zeros_struct (ast , strct , zerocopy_crate) , Data :: Enum (enm) => derive_from_zeros_enum (ast , enm , zerocopy_crate) ? , Data :: Union (unn) => derive_from_zeros_union (ast , unn , zerocopy_crate) , } ; Ok (IntoIterator :: into_iter ([try_from_bytes , from_zeros]) . collect ()) }
};
}
