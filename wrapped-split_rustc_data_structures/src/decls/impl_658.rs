macro_rules! deps {
    () => {
        UnordMap!();
    };
}

macro_rules! impl_658 {
    () => {
        deps!();
        impl < K : Hash + Eq , V > Extend < (K , V) > for UnordMap < K , V > { # [inline] fn extend < T : IntoIterator < Item = (K , V) > > (& mut self , iter : T) { self . inner . extend (iter) } }
    };
}

impl_658!();