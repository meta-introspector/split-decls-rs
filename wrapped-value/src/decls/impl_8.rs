macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Name { # [doc = " Create a new name."] pub fn new (name : impl AsRef < str >) -> Self { Self (name . as_ref () . into ()) } # [doc = " Get the name as a string."] # [must_use] pub fn as_str (& self) -> & str { & self . 0 } }
    };
}

impl_8!()