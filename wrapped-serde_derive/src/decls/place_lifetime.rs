macro_rules! place_lifetime {
    () => {
        # [cfg (feature = "deserialize_in_place")] fn place_lifetime () -> syn :: LifetimeParam { syn :: LifetimeParam { attrs : Vec :: new () , lifetime : syn :: Lifetime :: new ("'place" , Span :: call_site ()) , colon_token : None , bounds : Punctuated :: new () , } }
    };
}

place_lifetime!()