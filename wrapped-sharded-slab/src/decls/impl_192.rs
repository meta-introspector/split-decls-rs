macro_rules! deps {
    () => {
        Entry!();
        Config!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl < T , C > PartialEq < T > for Entry < '_ , T , C > where T : PartialEq < T > , C : cfg :: Config , { fn eq (& self , other : & T) -> bool { self . value () . eq (other) } }
    };
}

impl_192!()