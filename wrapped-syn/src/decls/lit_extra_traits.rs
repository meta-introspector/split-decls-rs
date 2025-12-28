macro_rules! deps {
    () => {
        TokenMarker!();
    };
}

macro_rules! lit_extra_traits {
    () => {
        deps!();
        macro_rules ! lit_extra_traits { ($ ty : ident) => { # [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl Clone for $ ty { fn clone (& self) -> Self { $ ty { repr : self . repr . clone () , } } } # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl PartialEq for $ ty { fn eq (& self , other : & Self) -> bool { self . repr . token . to_string () == other . repr . token . to_string () } } # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl Hash for $ ty { fn hash < H > (& self , state : & mut H) where H : Hasher , { self . repr . token . to_string () . hash (state) ; } } # [cfg (feature = "parsing")] pub_if_not_doc ! { # [doc (hidden)] # [allow (non_snake_case)] pub fn $ ty (marker : lookahead :: TokenMarker) -> $ ty { match marker { } } } } ; }
    };
}

lit_extra_traits!()