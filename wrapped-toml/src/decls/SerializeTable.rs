macro_rules! deps {
    () => {
        Style!();
    };
}

macro_rules! SerializeTable {
    () => {
        deps!();
        # [doc (hidden)] pub struct SerializeTable < 'd > { dst : & 'd mut String , seen_value : bool , key : Option < String > , style : Style , }
    };
}

SerializeTable!();