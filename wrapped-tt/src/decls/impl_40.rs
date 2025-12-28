macro_rules! deps {
    () => {
        Ident!();
        IdentIsRaw!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < S > Ident < S > { pub fn new (text : & str , span : S) -> Self { let (is_raw , text) = IdentIsRaw :: split_from_symbol (text) ; Ident { sym : Symbol :: intern (text) , span , is_raw } } }
    };
}

impl_40!()