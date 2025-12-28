macro_rules! deps {
    () => {
        OwnedRefMut!();
        Clear!();
        Config!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < T , C > PartialEq < T > for OwnedRefMut < T , C > where T : PartialEq < T > + Clear + Default , C : cfg :: Config , { fn eq (& self , other : & T) -> bool { * self . value () == * other } }
    };
}

impl_40!();