macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < 'a , K : Ord , V > Entry < 'a , K , V > { # [doc = " Returns a reference to this entry's key."] pub fn key (& self) -> & K { match * self { Entry :: Vacant (ref e) => e . key () , Entry :: Occupied (ref e) => e . key () , } } # [doc = " Ensures a value is in the entry by inserting the default if empty, and"] # [doc = " returns a mutable reference to the value in the entry."] pub fn or_insert (self , default : V) -> & 'a mut V { match self { Entry :: Vacant (entry) => entry . insert (default) , Entry :: Occupied (entry) => entry . into_mut () , } } # [doc = " Ensures a value is in the entry by inserting the result of the default"] # [doc = " function if empty, and returns a mutable reference to the value in the"] # [doc = " entry."] pub fn or_insert_with < F > (self , default : F) -> & 'a mut V where F : FnOnce () -> V , { match self { Entry :: Vacant (entry) => entry . insert (default ()) , Entry :: Occupied (entry) => entry . into_mut () , } } }
    };
}

impl_25!();