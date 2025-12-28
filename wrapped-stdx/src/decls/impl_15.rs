macro_rules! deps {
    () => {
        Downcast!();
        IntoBox!();
        OccupiedEntry!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < 'map , A : ? Sized + Downcast , V : IntoBox < A > > OccupiedEntry < 'map , A , V > { # [doc = " Converts the `OccupiedEntry` into a mutable reference to the value in the entry"] # [doc = " with a lifetime bound to the collection itself"] # [inline] # [must_use] pub fn into_mut (self) -> & 'map mut V { unsafe { self . inner . into_mut () . downcast_unchecked_mut () } } }
    };
}

impl_15!();