macro_rules! deps {
    () => {
        BSTR!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < T : AsRef < str > + ? Sized > PartialEq < T > for BSTR { fn eq (& self , other : & T) -> bool { self . iter () . copied () . eq (other . as_ref () . encode_utf16 ()) } }
    };
}

impl_16!();