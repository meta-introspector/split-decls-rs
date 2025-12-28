macro_rules! deps {
    () => {
        Lifetime!();
    };
}

macro_rules! impl_398 {
    () => {
        deps!();
        impl Lifetime { # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the lifetime does not conform to the bulleted rules above."] # [doc = ""] # [doc = " # Invocation"] # [doc = ""] # [doc = " ```"] # [doc = " # use proc_macro2::Span;"] # [doc = " # use syn::Lifetime;"] # [doc = " #"] # [doc = " # fn f() -> Lifetime {"] # [doc = " Lifetime::new(\"'a\", Span::call_site())"] # [doc = " # }"] # [doc = " ```"] pub fn new (symbol : & str , span : Span) -> Self { if ! symbol . starts_with ('\'') { panic ! ("lifetime name must start with apostrophe as in \"'a\", got {:?}" , symbol) ; } if symbol == "'" { panic ! ("lifetime name must not be empty") ; } if ! crate :: ident :: xid_ok (& symbol [1 ..]) { panic ! ("{:?} is not a valid lifetime name" , symbol) ; } Lifetime { apostrophe : span , ident : Ident :: new (& symbol [1 ..] , span) , } } pub fn span (& self) -> Span { self . apostrophe . join (self . ident . span ()) . unwrap_or (self . apostrophe) } pub fn set_span (& mut self , span : Span) { self . apostrophe = span ; self . ident . set_span (span) ; } }
    };
}

impl_398!()