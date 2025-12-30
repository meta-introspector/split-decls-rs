// Generated macro for place_lifetime (function)
macro_rules! Depcrate_deplace_lifetime {
() => {
// Module: crate::de
// Provides: {"place_lifetime"}
// Dependencies: {}
# [cfg (feature = "deserialize_in_place")] fn place_lifetime () -> syn :: LifetimeParam { syn :: LifetimeParam { attrs : Vec :: new () , lifetime : syn :: Lifetime :: new ("'place" , Span :: call_site ()) , colon_token : None , bounds : Punctuated :: new () , } }
};
}
