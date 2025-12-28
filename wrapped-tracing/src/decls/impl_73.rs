macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl cmp :: PartialEq for Inner { fn eq (& self , other : & Self) -> bool { self . id == other . id } }
    };
}

impl_73!();