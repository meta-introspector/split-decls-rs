macro_rules! deps {
    () => {
        Error!();
        Table!();
        Buffer!();
        Style!();
        SerializeDocumentTable!();
    };
}

macro_rules! impl_287 {
    () => {
        deps!();
        impl < 'd > SerializeDocumentTable < 'd > { pub (crate) fn map (buf : & 'd mut Buffer , table : Table , style : Style) -> Result < Self , Error > { Ok (Self { buf , table , key : None , style , }) } fn end (self) -> Result < & 'd mut Buffer , Error > { self . buf . push (self . table) ; Ok (self . buf) } }
    };
}

impl_287!()