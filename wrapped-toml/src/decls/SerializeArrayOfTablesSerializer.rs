macro_rules! deps {
    () => {
        Style!();
        Buffer!();
        Table!();
    };
}

macro_rules! SerializeArrayOfTablesSerializer {
    () => {
        deps!();
        # [doc (hidden)] pub (crate) struct SerializeArrayOfTablesSerializer < 'd > { buf : & 'd mut Buffer , parent : Table , key : String , style : Style , }
    };
}

SerializeArrayOfTablesSerializer!()