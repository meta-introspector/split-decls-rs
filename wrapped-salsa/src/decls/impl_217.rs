macro_rules! deps {
    () => {
        Lookup!();
    };
}

macro_rules! impl_217 {
    () => {
        deps!();
        impl < const N : usize , A : Hash + Eq + PartialEq < T > + Clone + Lookup < T > , T > Lookup < Vec < T > > for [A ; N] { fn into_owned (self) -> Vec < T > { self . into_iter () . map (| a | Lookup :: into_owned (a . clone ())) . collect () } }
    };
}

impl_217!()