macro_rules! deps {
    () => {
        OwnedRefMut!();
        Clear!();
        Config!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        unsafe impl < T , C > Sync for OwnedRefMut < T , C > where T : Sync + Clear + Default , C : cfg :: Config , { }
    };
}

impl_41!()