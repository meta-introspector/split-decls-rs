macro_rules! deps {
    () => {
        Qualifiers!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Qualifiers { fn from_ident (ident : & Ident) -> Self { match ident . to_string () . as_str () { "async" => Qualifiers :: Async , "unsafe" => Qualifiers :: Unsafe , "extern" => Qualifiers :: Extern , _ => Qualifiers :: None , } } }
    };
}

impl_11!()