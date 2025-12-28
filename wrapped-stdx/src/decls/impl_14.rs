macro_rules! deps {
    () => {
        IntoBox!();
        Entry!();
        Downcast!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < 'map , A : ? Sized + Downcast , V : IntoBox < A > > Entry < 'map , A , V > { # [doc = " Ensures a value is in the entry by inserting the result of the default function if"] # [doc = " empty, and returns a mutable reference to the value in the entry."] # [inline] pub fn or_insert_with < F : FnOnce () -> V > (self , default : F) -> & 'map mut V { match self { Entry :: Occupied (inner) => inner . into_mut () , Entry :: Vacant (inner) => inner . insert (default ()) , } } }
    };
}

impl_14!()