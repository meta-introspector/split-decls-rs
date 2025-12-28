macro_rules! deps {
    () => {
        Downcast!();
        VacantEntry!();
        IntoBox!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < 'map , A : ? Sized + Downcast , V : IntoBox < A > > VacantEntry < 'map , A , V > { # [doc = " Sets the value of the entry with the `VacantEntry`'s key,"] # [doc = " and returns a mutable reference to it"] # [inline] pub fn insert (self , value : V) -> & 'map mut V { unsafe { self . inner . insert (value . into_box ()) . downcast_unchecked_mut () } } }
    };
}

impl_16!();