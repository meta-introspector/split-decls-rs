macro_rules! deps {
    () => {
        OwnedEntry!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < T , C > PartialEq < T > for OwnedEntry < T , C > where T : PartialEq < T > , C : cfg :: Config , { fn eq (& self , other : & T) -> bool { * self . value () == * other } }
    };
}

impl_31!()