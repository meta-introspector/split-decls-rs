macro_rules! deps {
    () => {
        OwnedRef!();
        Config!();
        Clear!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < T , C > PartialEq < T > for OwnedRef < T , C > where T : PartialEq < T > + Clear + Default , C : cfg :: Config , { fn eq (& self , other : & T) -> bool { * self . value () == * other } }
    };
}

impl_32!()