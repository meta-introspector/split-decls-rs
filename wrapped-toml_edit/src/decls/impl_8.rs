macro_rules! deps {
    () => {
        Value!();
        Item!();
        Array!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < V : Into < Value > > Extend < V > for Array { fn extend < T : IntoIterator < Item = V > > (& mut self , iter : T) { for value in iter { self . push_formatted (value . into ()) ; } } }
    };
}

impl_8!();