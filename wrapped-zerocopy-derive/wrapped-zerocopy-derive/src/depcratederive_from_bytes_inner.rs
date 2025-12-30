// Generated macro for derive_from_bytes_inner (function)
macro_rules! Depcratederive_from_bytes_inner {
() => {
// Module: crate
// Provides: {"derive_from_bytes_inner"}
// Dependencies: {}
fn derive_from_bytes_inner (ast : & DeriveInput , top_level : Trait , zerocopy_crate : & Path ,) -> Result < TokenStream , Error > { let from_zeros = derive_from_zeros_inner (ast , top_level , zerocopy_crate) ? ; let from_bytes = match & ast . data { Data :: Struct (strct) => derive_from_bytes_struct (ast , strct , zerocopy_crate) , Data :: Enum (enm) => derive_from_bytes_enum (ast , enm , zerocopy_crate) ? , Data :: Union (unn) => derive_from_bytes_union (ast , unn , zerocopy_crate) , } ; Ok (IntoIterator :: into_iter ([from_zeros , from_bytes]) . collect ()) }
};
}
