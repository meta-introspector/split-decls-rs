macro_rules! deps {
    () => {
        Equal!();
        Less!();
        Ord!();
        InternalMarker!();
        Greater!();
    };
}

macro_rules! Cmp {
    () => {
        deps!();
        # [doc = " A **type operator** for comparing `Self` and `Rhs`. It provides a similar functionality to"] # [doc = " the function"] # [doc = " [`core::cmp::Ord::cmp`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)"] # [doc = " but for types."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " use typenum::{Cmp, Ord, N3, P2, P5};"] # [doc = " use std::cmp::Ordering;"] # [doc = ""] # [doc = " assert_eq!(<P2 as Cmp<N3>>::Output::to_ordering(), Ordering::Greater);"] # [doc = " assert_eq!(<P2 as Cmp<P2>>::Output::to_ordering(), Ordering::Equal);"] # [doc = " assert_eq!(<P2 as Cmp<P5>>::Output::to_ordering(), Ordering::Less);"] pub trait Cmp < Rhs = Self > { # [doc = " The result of the comparison. It should only ever be one of `Greater`, `Less`, or `Equal`."] type Output ; # [doc (hidden)] fn compare < IM : InternalMarker > (& self , _ : & Rhs) -> Self :: Output ; }
    };
}

Cmp!()