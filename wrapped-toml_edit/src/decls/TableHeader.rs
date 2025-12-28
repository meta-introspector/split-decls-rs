macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! TableHeader {
    () => {
        deps!();
        struct TableHeader { path : Vec < Key > , key : Option < Key > , span : toml_parser :: Span , is_array : bool , }
    };
}

TableHeader!();