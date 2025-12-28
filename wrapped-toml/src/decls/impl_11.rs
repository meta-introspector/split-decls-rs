macro_rules! deps {
    () => {
        Map!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < K : Eq + Hash , V : PartialEq > PartialEq for Map < K , V > { # [inline] fn eq (& self , other : & Self) -> bool { self . map . eq (& other . map) } }
    };
}

impl_11!();