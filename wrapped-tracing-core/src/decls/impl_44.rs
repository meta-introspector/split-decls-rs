macro_rules! deps {
    () => {
        Identifier!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl PartialEq for Identifier { fn eq (& self , other : & Identifier) -> bool { core :: ptr :: eq (self . 0 as * const _ as * const () , other . 0 as * const _ as * const () ,) } }
    };
}

impl_44!();