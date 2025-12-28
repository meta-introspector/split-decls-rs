macro_rules! deps {
    () => {
        Document!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < S : AsRef < str > > Document < S > { # [doc = " Access the raw, unparsed document"] pub fn raw (& self) -> & str { self . raw . as_ref () } }
    };
}

impl_32!()