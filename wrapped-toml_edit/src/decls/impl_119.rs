macro_rules! deps {
    () => {
        Item!();
        Value!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < V : Into < Value > > From < V > for Item { fn from (s : V) -> Self { Self :: Value (s . into ()) } }
    };
}

impl_119!();