macro_rules! deps {
    () => {
        Table!();
        Style!();
        Buffer!();
    };
}

macro_rules! ArrayOfTablesSerializer {
    () => {
        deps!();
        pub (crate) struct ArrayOfTablesSerializer < 'd > { buf : & 'd mut Buffer , parent : Table , key : String , style : Style , }
    };
}

ArrayOfTablesSerializer!();