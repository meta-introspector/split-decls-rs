macro_rules! deps {
    () => {
        Buffer!();
        Table!();
        Style!();
    };
}

macro_rules! SerializeDocumentTable {
    () => {
        deps!();
        # [doc (hidden)] pub struct SerializeDocumentTable < 'd > { buf : & 'd mut Buffer , table : Table , key : Option < String > , style : Style , }
    };
}

SerializeDocumentTable!();