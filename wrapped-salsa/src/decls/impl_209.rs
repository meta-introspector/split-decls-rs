macro_rules! deps {
    () => {
        Lookup!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl < 'a , T > Lookup < Arc < T > > for & 'a T where T : ? Sized + Hash + Eq , Arc < T > : From < & 'a T > , { fn into_owned (self) -> Arc < T > { Arc :: from (self) } }
    };
}

impl_209!()