macro_rules! deps {
    () => {
        ByteOrder!();
    };
}

macro_rules! impl_fmt_trait {
    () => {
        deps!();
        macro_rules ! impl_fmt_trait { ($ name : ident , $ native : ident , $ trait : ident) => { impl < O : ByteOrder > $ trait for $ name < O > { # [inline (always)] fn fmt (& self , f : & mut Formatter <'_ >) -> fmt :: Result { $ trait :: fmt (& self . get () , f) } } } ; }
    };
}

impl_fmt_trait!()