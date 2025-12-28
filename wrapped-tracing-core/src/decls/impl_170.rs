macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl PartialEq for Field { fn eq (& self , other : & Self) -> bool { self . callsite () == other . callsite () && self . i == other . i } }
    };
}

impl_170!()