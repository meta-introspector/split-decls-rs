macro_rules! deps {
    () => {
        IntoSpans!();
        Result!();
    };
}

macro_rules! define_punctuation_structs {
    () => {
        deps!();
        macro_rules ! define_punctuation_structs { ($ ($ token : literal pub struct $ name : ident /$ len : tt # [doc = $ usage : literal]) *) => { $ (# [cfg_attr (not (doc) , repr (transparent))] # [allow (unknown_lints , repr_transparent_non_zst_fields)] # [doc = concat ! ('`' , $ token , '`')] # [doc = ""] # [doc = " Usage:"] # [doc = concat ! ($ usage , '.')] # [doc = ""] # [doc = " Don't try to remember the name of this type &mdash; use the"] # [doc = " [`Token!`] macro instead."] # [doc = ""] # [doc = " [`Token!`]: crate::token"] pub struct $ name { pub spans : [Span ; $ len] , } # [doc (hidden)] # [allow (non_snake_case)] pub fn $ name < S : IntoSpans < [Span ; $ len] >> (spans : S) -> $ name { $ name { spans : spans . into_spans () , } } impl std :: default :: Default for $ name { fn default () -> Self { $ name { spans : [Span :: call_site () ; $ len] , } } } # [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl Copy for $ name { } # [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl Clone for $ name { fn clone (& self) -> Self { * self } } # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl Debug for $ name { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str (stringify ! ($ name)) } } # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl cmp :: Eq for $ name { } # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl PartialEq for $ name { fn eq (& self , _other : &$ name) -> bool { true } } # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl Hash for $ name { fn hash < H : Hasher > (& self , _state : & mut H) { } } impl_deref_if_len_is_1 ! ($ name /$ len) ;) * } ; }
    };
}

define_punctuation_structs!()