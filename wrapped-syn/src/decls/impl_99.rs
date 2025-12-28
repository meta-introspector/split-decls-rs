macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < 'a > PartialOrd for Cursor < 'a > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { if same_buffer (* self , * other) { Some (cmp_assuming_same_buffer (* self , * other)) } else { None } } }
    };
}

impl_99!();