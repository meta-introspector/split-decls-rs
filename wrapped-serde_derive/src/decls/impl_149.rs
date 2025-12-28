macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl PartialEq < Symbol > for & Path { fn eq (& self , word : & Symbol) -> bool { self . is_ident (word . 0) } }
    };
}

impl_149!();