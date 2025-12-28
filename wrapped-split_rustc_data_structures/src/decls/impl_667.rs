macro_rules! deps {
    () => {
        UnordBag!();
    };
}

macro_rules! impl_667 {
    () => {
        deps!();
        impl < T > Extend < T > for UnordBag < T > { fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { self . inner . extend (iter) } }
    };
}

impl_667!();