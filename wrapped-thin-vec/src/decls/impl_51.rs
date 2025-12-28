macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < T : Clone > From < & [T] > for ThinVec < T > { # [doc = " Allocate a `ThinVec<T>` and fill it by cloning `s`'s items."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use thin_vec::{ThinVec, thin_vec};"] # [doc = ""] # [doc = " assert_eq!(ThinVec::from(&[1, 2, 3][..]), thin_vec![1, 2, 3]);"] # [doc = " ```"] fn from (s : & [T]) -> ThinVec < T > { s . iter () . cloned () . collect () } }
    };
}

impl_51!();