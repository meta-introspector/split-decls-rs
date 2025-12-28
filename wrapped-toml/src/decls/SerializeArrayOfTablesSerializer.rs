macro_rules! deps {
    () => {
        Table!();
        Buffer!();
        Style!();
    };
}

macro_rules! SerializeArrayOfTablesSerializer {
    () => {
        deps!();
        # [doc (hidden)] pub (crate) struct SerializeArrayOfTablesSerializer < 'd > { buf : & 'd mut Buffer , parent : Table , key : String , style : Style , }
    };
}

SerializeArrayOfTablesSerializer!();