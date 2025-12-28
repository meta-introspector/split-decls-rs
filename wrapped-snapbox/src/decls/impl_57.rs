macro_rules! deps {
    () => {
        FilterSet!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl FilterSet { const REDACTIONS : usize = 1 << 0 ; const NEWLINES : usize = 1 << 1 ; const PATHS : usize = 1 << 2 ; const UNORDERED : usize = 1 << 3 ; fn set (& mut self , flag : usize) -> & mut Self { self . flags |= flag ; self } const fn is_set (& self , flag : usize) -> bool { self . flags & flag != 0 } }
    };
}

impl_57!();