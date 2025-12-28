macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl PartialEq < & HSTRING > for str { fn eq (& self , other : & & HSTRING) -> bool { * * other == * self } }
    };
}

impl_48!();