macro_rules! deps {
    () => {
        TaggedRef!();
        Aligned!();
        Tag!();
    };
}

macro_rules! impl_586 {
    () => {
        deps!();
        impl < P , T > PartialEq for TaggedRef < '_ , P , T > where P : Aligned + ? Sized , T : Tag , { # [inline] # [allow (ambiguous_wide_pointer_comparisons)] fn eq (& self , other : & Self) -> bool { self . packed == other . packed } }
    };
}

impl_586!();