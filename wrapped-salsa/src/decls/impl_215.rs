macro_rules! deps {
    () => {
        Lookup!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl < A : Hash + Eq + PartialEq < T > + Clone + Lookup < T > , T > Lookup < Vec < T > > for & [A] { fn into_owned (self) -> Vec < T > { self . iter () . map (| a | Lookup :: into_owned (a . clone ())) . collect () } }
    };
}

impl_215!();