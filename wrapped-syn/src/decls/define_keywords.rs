macro_rules! deps {
    () => {
        IntoSpans!();
        ParseStream!();
        Parse!();
        Result!();
        Cursor!();
    };
}

macro_rules! define_keywords {
    () => {
        deps!();
        macro_rules ! define_keywords { ($ ($ token : literal pub struct $ name : ident) *) => { $ (# [doc = concat ! ('`' , $ token , '`')] # [doc = ""] # [doc = " Don't try to remember the name of this type &mdash; use the"] # [doc = " [`Token!`] macro instead."] # [doc = ""] # [doc = " [`Token!`]: crate::token"] pub struct $ name { pub span : Span , } # [doc (hidden)] # [allow (non_snake_case)] pub fn $ name < S : IntoSpans < Span >> (span : S) -> $ name { $ name { span : span . into_spans () , } } impl std :: default :: Default for $ name { fn default () -> Self { $ name { span : Span :: call_site () , } } } # [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl Copy for $ name { } # [cfg (feature = "clone-impls")] # [cfg_attr (docsrs , doc (cfg (feature = "clone-impls")))] impl Clone for $ name { fn clone (& self) -> Self { * self } } # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl Debug for $ name { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str (stringify ! ($ name)) } } # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl cmp :: Eq for $ name { } # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl PartialEq for $ name { fn eq (& self , _other : &$ name) -> bool { true } } # [cfg (feature = "extra-traits")] # [cfg_attr (docsrs , doc (cfg (feature = "extra-traits")))] impl Hash for $ name { fn hash < H : Hasher > (& self , _state : & mut H) { } } # [cfg (feature = "printing")] # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl ToTokens for $ name { fn to_tokens (& self , tokens : & mut TokenStream) { printing :: keyword ($ token , self . span , tokens) ; } } # [cfg (feature = "parsing")] # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl Parse for $ name { fn parse (input : ParseStream) -> Result < Self > { Ok ($ name { span : parsing :: keyword (input , $ token) ?, }) } } # [cfg (feature = "parsing")] impl Token for $ name { fn peek (cursor : Cursor) -> bool { parsing :: peek_keyword (cursor , $ token) } fn display () -> &'static str { concat ! ("`" , $ token , "`") } } # [cfg (feature = "parsing")] impl private :: Sealed for $ name { }) * } ; }
    };
}

define_keywords!();