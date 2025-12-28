macro_rules! deps {
    () => {
        WithCachedTypeInfo!();
    };
}

macro_rules! impl_418 {
    () => {
        deps!();
        impl < T : PartialEq > PartialEq for WithCachedTypeInfo < T > { # [inline] fn eq (& self , other : & Self) -> bool { self . internee . eq (& other . internee) } }
    };
}

impl_418!()