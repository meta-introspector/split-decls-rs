macro_rules! deps {
    () => {
        FileNameDisplay!();
        FileName!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < 'a > FileNameDisplay < 'a > { pub fn to_string_lossy (& self) -> Cow < 'a , str > { match self . inner { FileName :: Real (inner) => inner . to_string_lossy (self . display_pref) , _ => Cow :: from (self . to_string ()) , } } }
    };
}

impl_33!()