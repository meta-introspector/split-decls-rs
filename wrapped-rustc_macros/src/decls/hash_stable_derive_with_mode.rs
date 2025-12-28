macro_rules! deps {
    () => {
        HashStableMode!();
    };
}

macro_rules! hash_stable_derive_with_mode {
    () => {
        deps!();
        fn hash_stable_derive_with_mode (mut s : synstructure :: Structure < '_ > , mode : HashStableMode ,) -> proc_macro2 :: TokenStream { let generic : syn :: GenericParam = match mode { HashStableMode :: Normal => parse_quote ! ('__ctx) , HashStableMode :: Generic | HashStableMode :: NoContext => parse_quote ! (__CTX) , } ; s . add_bounds (match mode { HashStableMode :: Normal | HashStableMode :: Generic => synstructure :: AddBounds :: Generics , HashStableMode :: NoContext => synstructure :: AddBounds :: Fields , }) ; match mode { HashStableMode :: Normal => { } HashStableMode :: Generic => { s . add_where_predicate (parse_quote ! { __CTX : crate :: HashStableContext }) ; } HashStableMode :: NoContext => { } } s . add_impl_generic (generic) ; let discriminant = hash_stable_discriminant (& mut s) ; let body = hash_stable_body (& mut s) ; let context : syn :: Type = match mode { HashStableMode :: Normal => { parse_quote ! (:: rustc_query_system :: ich :: StableHashingContext <'__ctx >) } HashStableMode :: Generic | HashStableMode :: NoContext => parse_quote ! (__CTX) , } ; s . bound_impl (quote ! (:: rustc_data_structures :: stable_hasher :: HashStable < # context >) , quote ! { # [inline] fn hash_stable (& self , __hcx : & mut # context , __hasher : & mut :: rustc_data_structures :: stable_hasher :: StableHasher) { # discriminant match * self { # body } } } ,) }
    };
}

hash_stable_derive_with_mode!()