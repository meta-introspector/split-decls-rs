macro_rules! deps {
    () => {
        Table!();
        Style!();
        Buffer!();
    };
}

macro_rules! SerializeDocumentTupleVariant {
    () => {
        deps!();
        # [doc (hidden)] pub struct SerializeDocumentTupleVariant < 'd > { buf : & 'd mut Buffer , table : Table , seen_value : bool , style : Style , }
    };
}

SerializeDocumentTupleVariant!()