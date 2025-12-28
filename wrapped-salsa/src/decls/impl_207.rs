macro_rules! deps {
    () => {
        Lookup!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl < 'a , T > Lookup < Box < T > > for & 'a T where T : ? Sized + Hash + Eq , Box < T > : From < & 'a T > , { fn into_owned (self) -> Box < T > { Box :: from (self) } }
    };
}

impl_207!()