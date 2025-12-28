macro_rules! deps {
    () => {
        DeTypeGenerics!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl < 'a > ToTokens for DeTypeGenerics < 'a > { fn to_tokens (& self , tokens : & mut TokenStream) { de_type_generics_to_tokens (self . 0 . generics . clone () , & self . 0 . borrowed , tokens) ; } }
    };
}

impl_223!();