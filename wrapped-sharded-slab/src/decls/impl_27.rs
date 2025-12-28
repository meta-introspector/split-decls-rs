macro_rules! deps {
    () => {
        RefMut!();
        Config!();
        Clear!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < T , C > PartialEq < T > for RefMut < '_ , T , C > where T : PartialEq < T > + Clear + Default , C : cfg :: Config , { fn eq (& self , other : & T) -> bool { self . value () . eq (other) } }
    };
}

impl_27!()