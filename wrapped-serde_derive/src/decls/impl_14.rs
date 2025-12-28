macro_rules! impl_14 {
    () => {
        impl private { fn ident (& self) -> Ident { Ident :: new (concat ! ("__private" , env ! ("CARGO_PKG_VERSION_PATCH")) , Span :: call_site () ,) } }
    };
}

impl_14!()