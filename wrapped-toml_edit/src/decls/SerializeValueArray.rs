macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! SerializeValueArray {
    () => {
        deps!();
        # [doc (hidden)] pub struct SerializeValueArray { values : Vec < crate :: Item > , }
    };
}

SerializeValueArray!()