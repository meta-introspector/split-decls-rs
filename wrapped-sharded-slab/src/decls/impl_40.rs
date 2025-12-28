macro_rules! deps {
    () => {
        Config!();
        OwnedRefMut!();
        Clear!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < T , C > PartialEq < T > for OwnedRefMut < T , C > where T : PartialEq < T > + Clear + Default , C : cfg :: Config , { fn eq (& self , other : & T) -> bool { * self . value () == * other } }
    };
}

impl_40!()