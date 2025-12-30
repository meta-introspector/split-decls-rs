// Generated macro for de_type_generics_to_tokens (function)
macro_rules! Depcrate_dede_type_generics_to_tokens {
() => {
// Module: crate::de
// Provides: {"de_type_generics_to_tokens"}
// Dependencies: {}
fn de_type_generics_to_tokens (mut generics : syn :: Generics , borrowed : & BorrowedLifetimes , tokens : & mut TokenStream ,) { if borrowed . de_lifetime_param () . is_some () { let def = syn :: LifetimeParam { attrs : Vec :: new () , lifetime : syn :: Lifetime :: new ("'de" , Span :: call_site ()) , colon_token : None , bounds : Punctuated :: new () , } ; generics . params = Some (syn :: GenericParam :: Lifetime (def)) . into_iter () . chain (generics . params) . collect () ; } let (_ , ty_generics , _) = generics . split_for_impl () ; ty_generics . to_tokens (tokens) ; }
};
}
