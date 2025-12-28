macro_rules! deps {
    () => {
        Key!();
        Item!();
        Value!();
        InlineTable!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < K : Into < Key > , V : Into < Value > > Extend < (K , V) > for InlineTable { fn extend < T : IntoIterator < Item = (K , V) > > (& mut self , iter : T) { for (key , value) in iter { let key = key . into () ; let value = Item :: Value (value . into ()) ; self . items . insert (key , value) ; } } }
    };
}

impl_94!()