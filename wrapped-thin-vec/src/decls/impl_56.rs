macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < T > From < ThinVec < T > > for Vec < T > { # [doc = " Convert a `ThinVec` into a `std::Vec`."] # [doc = ""] # [doc = " **NOTE:** this must reallocate to change the layout!"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use thin_vec::{ThinVec, thin_vec};"] # [doc = ""] # [doc = " let b: ThinVec<i32> = thin_vec![1, 2, 3];"] # [doc = " assert_eq!(Vec::from(b), vec![1, 2, 3]);"] # [doc = " ```"] fn from (s : ThinVec < T >) -> Self { s . into_iter () . collect () } }
    };
}

impl_56!()