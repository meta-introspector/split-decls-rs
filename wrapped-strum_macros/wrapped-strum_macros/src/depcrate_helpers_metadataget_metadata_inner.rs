// Generated macro for get_metadata_inner (function)
macro_rules! Depcrate_helpers_metadataget_metadata_inner {
() => {
// Module: crate::helpers::metadata
// Provides: {"get_metadata_inner"}
// Dependencies: {}
fn get_metadata_inner < 'a , T : Parse > (ident : & str , it : impl IntoIterator < Item = & 'a Attribute > ,) -> syn :: Result < Vec < T > > { it . into_iter () . filter (| attr | attr . path () . is_ident (ident)) . try_fold (Vec :: new () , | mut vec , attr | { vec . extend (attr . parse_args_with (Punctuated :: < T , Token ! [,] > :: parse_terminated) ?) ; Ok (vec) }) }
};
}
