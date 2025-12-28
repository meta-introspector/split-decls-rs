macro_rules! deps {
    () => {
        OwnedRefMut!();
        Clear!();
        Config!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        unsafe impl < T , C > Send for OwnedRefMut < T , C > where T : Sync + Clear + Default , C : cfg :: Config , { }
    };
}

impl_42!()