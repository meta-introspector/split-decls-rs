macro_rules! deps {
    () => {
        DeString!();
    };
}

macro_rules! TableHeader {
    () => {
        deps!();
        struct TableHeader < 'i > { path : Vec < Spanned < DeString < 'i > > > , key : Option < Spanned < DeString < 'i > > > , span : toml_parser :: Span , is_array : bool , }
    };
}

TableHeader!();