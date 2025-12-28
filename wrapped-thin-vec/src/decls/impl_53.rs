macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < T , const N : usize > From < [T ; N] > for ThinVec < T > { # [doc = " Allocate a `ThinVec<T>` and move `s`'s items into it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use thin_vec::{ThinVec, thin_vec};"] # [doc = ""] # [doc = " assert_eq!(ThinVec::from([1, 2, 3]), thin_vec![1, 2, 3]);"] # [doc = " ```"] fn from (s : [T ; N]) -> ThinVec < T > { core :: iter :: IntoIterator :: into_iter (s) . collect () } }
    };
}

impl_53!()