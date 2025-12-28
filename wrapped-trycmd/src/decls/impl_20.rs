macro_rules! deps {
    () => {
        JoinedArgs!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl JoinedArgs { # [cfg (test)] pub (crate) fn from_vec (inner : Vec < String >) -> Self { JoinedArgs { inner } } # [allow (clippy :: inherent_to_string_shadow_display)] fn to_string (& self) -> String { shlex :: join (self . inner . iter () . map (| s | s . as_str ())) } }
    };
}

impl_20!()