macro_rules! deps {
    () => {
        Segment!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl PartialEq for Segment { fn eq (& self , other : & Self) -> bool { self . score == other . score } }
    };
}

impl_203!();