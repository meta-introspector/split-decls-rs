macro_rules! deps {
    () => {
        BindStyle!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl ToTokens for BindStyle { fn to_tokens (& self , tokens : & mut TokenStream) { match self { BindStyle :: Move => { } BindStyle :: MoveMut => quote_spanned ! (Span :: call_site () => mut) . to_tokens (tokens) , BindStyle :: Ref => quote_spanned ! (Span :: call_site () => ref) . to_tokens (tokens) , BindStyle :: RefMut => quote_spanned ! (Span :: call_site () => ref mut) . to_tokens (tokens) , } } }
    };
}

impl_7!();