macro_rules! deps {
    () => {
        Name!();
        ConstValue!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl From < IndexMap < Name , ConstValue > > for ConstValue { fn from (f : IndexMap < Name , ConstValue >) -> Self { ConstValue :: Object (f) } }
    };
}

impl_36!()