macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl PartialEq for Span { # [doc = " Returns `true` if and only if this `Span` and `other` have the same range and data,"] # [doc = " regardless of `committed` status."] fn eq (& self , other : & Self) -> bool { self . range == other . range && self . data == other . data } }
    };
}

impl_12!();