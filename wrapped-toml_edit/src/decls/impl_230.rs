macro_rules! deps {
    () => {
        Key!();
        Table!();
        Item!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl < K : Into < Key > , V : Into < Item > > Extend < (K , V) > for Table { fn extend < T : IntoIterator < Item = (K , V) > > (& mut self , iter : T) { for (key , value) in iter { let key = key . into () ; let value = value . into () ; self . items . insert (key , value) ; } } }
    };
}

impl_230!()