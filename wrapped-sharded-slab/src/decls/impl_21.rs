macro_rules! deps {
    () => {
        Clear!();
        Config!();
        Ref!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < T , C > PartialEq < T > for Ref < '_ , T , C > where T : PartialEq < T > + Clear + Default , C : cfg :: Config , { fn eq (& self , other : & T) -> bool { * self . value () == * other } }
    };
}

impl_21!();