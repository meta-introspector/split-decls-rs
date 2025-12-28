macro_rules! deps {
    () => {
        ThinVec!();
        IntoIter!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < T : Clone > Clone for IntoIter < T > { # [allow (clippy :: into_iter_on_ref)] fn clone (& self) -> Self { self . as_slice () . into_iter () . cloned () . collect :: < ThinVec < _ > > () . into_iter () } }
    };
}

impl_70!()