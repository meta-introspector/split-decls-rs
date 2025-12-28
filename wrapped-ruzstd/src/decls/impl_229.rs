macro_rules! deps {
    () => {
        FseTableMode!();
        FSETable!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl FseTableMode < '_ > { pub fn as_ref (& self) -> & FSETable { match self { Self :: Predefined (t) => t , Self :: RepeateLast (t) => t , Self :: Encoded (t) => t , } } }
    };
}

impl_229!()