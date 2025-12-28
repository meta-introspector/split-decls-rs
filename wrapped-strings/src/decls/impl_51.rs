macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl PartialEq < & HSTRING > for String { fn eq (& self , other : & & HSTRING) -> bool { * * other == * * self } }
    };
}

impl_51!()