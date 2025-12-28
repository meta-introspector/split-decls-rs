macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl PartialOrd for SmolStr { fn partial_cmp (& self , other : & SmolStr) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_17!();