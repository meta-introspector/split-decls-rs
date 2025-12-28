macro_rules! deps {
    () => {
        Options!();
        AllowedOptions!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < A : AllowedOptions > Options < A > { pub fn persist (& self) -> bool { cfg ! (feature = "persistence") && self . persist . is_some () } }
    };
}

impl_50!()