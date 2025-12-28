macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < T > From < Vec < T > > for ThinVec < T > { # [doc = " Convert a `std::Vec` into a `ThinVec`."] # [doc = ""] # [doc = " **NOTE:** this must reallocate to change the layout!"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use thin_vec::{ThinVec, thin_vec};"] # [doc = ""] # [doc = " let b: Vec<i32> = vec![1, 2, 3];"] # [doc = " assert_eq!(ThinVec::from(b), thin_vec![1, 2, 3]);"] # [doc = " ```"] fn from (s : Vec < T >) -> Self { s . into_iter () . collect () } }
    };
}

impl_55!()