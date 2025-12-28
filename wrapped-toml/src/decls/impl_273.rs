macro_rules! deps {
    () => {
        Table!();
        Buffer!();
        SerializeArrayOfTablesSerializer!();
        Style!();
        Error!();
    };
}

macro_rules! impl_273 {
    () => {
        deps!();
        impl < 'd > SerializeArrayOfTablesSerializer < 'd > { pub (crate) fn seq (buf : & 'd mut Buffer , parent : Table , key : String , style : Style) -> Self { Self { buf , parent , key , style , } } fn end (self) -> Result < & 'd mut Buffer , Error > { Ok (self . buf) } }
    };
}

impl_273!();