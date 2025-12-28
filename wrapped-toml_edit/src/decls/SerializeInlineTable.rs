macro_rules! deps {
    () => {
        KeyValuePairs!();
        Key!();
    };
}

macro_rules! SerializeInlineTable {
    () => {
        deps!();
        # [doc (hidden)] pub struct SerializeInlineTable { items : crate :: table :: KeyValuePairs , key : Option < crate :: Key > , }
    };
}

SerializeInlineTable!();