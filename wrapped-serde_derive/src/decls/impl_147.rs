macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl PartialEq < Symbol > for & Ident { fn eq (& self , word : & Symbol) -> bool { * self == word . 0 } }
    };
}

impl_147!();