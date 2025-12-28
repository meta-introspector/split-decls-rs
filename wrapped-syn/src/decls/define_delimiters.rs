macro_rules! deps {
    () => {
        IntoSpans!();
        Result!();
    };
}

macro_rules! define_delimiters {
    () => {
        deps!();
        macro_rules ! define_delimiters { ($ ($ delim : ident pub struct $ name : ident # [$ doc : meta]) *) => { $ (# [$ doc] pub struct $ name { pub span : DelimSpan , } # [doc (hidden)] # [allow (non_snake_case)] pub fn $ name < S : IntoSpans < DelimSpan >> (span : S) -> $ name { $ name { span : span . into_spans () , } } impl std :: default :: Default for $ name { fn default () -> Self { $ name (Span :: call_site ()) } } # [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl Copy for $ name { } # [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl Clone for $ name { fn clone (& self) -> Self { * self } } # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl Debug for $ name { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str (stringify ! ($ name)) } } # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl cmp :: Eq for $ name { } # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl PartialEq for $ name { fn eq (& self , _other : &$ name) -> bool { true } } # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl Hash for $ name { fn hash < H : Hasher > (& self , _state : & mut H) { } } impl $ name { # [cfg (feature = "printing")] # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] pub fn surround < F > (& self , tokens : & mut TokenStream , f : F) where F : FnOnce (& mut TokenStream) , { let mut inner = TokenStream :: new () ; f (& mut inner) ; printing :: delim (Delimiter ::$ delim , self . span . join () , tokens , inner) ; } } # [cfg (feature = "parsing")] impl private :: Sealed for $ name { }) * } ; }
    };
}

define_delimiters!()