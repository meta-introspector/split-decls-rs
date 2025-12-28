macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl PartialEq < str > for HSTRING { fn eq (& self , other : & str) -> bool { self . iter () . copied () . eq (other . encode_utf16 ()) } }
    };
}

impl_43!()