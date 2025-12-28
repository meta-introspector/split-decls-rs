macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < T > AsRef < [T] > for IntoIter < T > { fn as_ref (& self) -> & [T] { self . as_slice () } }
    };
}

impl_69!()