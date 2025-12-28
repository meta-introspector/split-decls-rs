macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! Inline {
    () => {
        deps!();
        # [doc = " Output of [`str!`][crate::str!]"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct Inline { # [doc (hidden)] pub position : Position , # [doc (hidden)] pub data : & 'static str , }
    };
}

Inline!();