macro_rules! deps {
    () => {
        Style!();
    };
}

macro_rules! SerializeValueArray {
    () => {
        deps!();
        # [doc (hidden)] pub struct SerializeValueArray < 'd > { dst : & 'd mut String , seen_value : bool , style : Style , len : Option < usize > , }
    };
}

SerializeValueArray!()