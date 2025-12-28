macro_rules! deps {
    () => {
        Hygiene!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl Hygiene { # [doc = " Generates an identifier similar to `text` but"] # [doc = " distinct from any identifiers that appear in the user's"] # [doc = " code."] pub (crate) fn ident (& self , text : impl AsRef < str >) -> syn :: Ident { let mut buffer = format ! ("{}_" , text . as_ref ()) ; while self . user_tokens . contains (& buffer) { buffer . push ('_') ; } syn :: Ident :: new (& buffer , proc_macro2 :: Span :: call_site ()) } # [doc = " Generates an identifier similar to `text` but distinct from any identifiers"] # [doc = " that appear in the user's code."] # [doc = ""] # [doc = " The identifier must be unique relative to the `scope` identifier."] pub (crate) fn scoped_ident (& self , scope : & syn :: Ident , text : & str) -> syn :: Ident { self . ident (format ! ("{scope}_{text}")) } }
    };
}

impl_31!()