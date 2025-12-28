macro_rules! deps {
    () => {
        UnordSet!();
    };
}

macro_rules! impl_650 {
    () => {
        deps!();
        impl < V : Hash + Eq > Extend < V > for UnordSet < V > { # [inline] fn extend < T : IntoIterator < Item = V > > (& mut self , iter : T) { self . inner . extend (iter) } }
    };
}

impl_650!();