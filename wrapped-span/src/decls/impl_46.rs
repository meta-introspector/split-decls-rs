macro_rules! deps {
    () => {
        AstIdMap!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl PartialEq for AstIdMap { fn eq (& self , other : & Self) -> bool { self . arena == other . arena } }
    };
}

impl_46!()