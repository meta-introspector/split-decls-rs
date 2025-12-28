macro_rules! deps {
    () => {
        Array!();
        Value!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < V : Into < Self > > From < Vec < V > > for Value { fn from (val : Vec < V >) -> Self { Self :: Array (val . into_iter () . map (| v | v . into ()) . collect ()) } }
    };
}

impl_58!()