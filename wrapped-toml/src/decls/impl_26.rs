macro_rules! deps {
    () => {
        VacantEntry!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < 'a , K : Ord , V > VacantEntry < 'a , K , V > { # [doc = " Gets a reference to the key that would be used when inserting a value"] # [doc = " through the `VacantEntry`."] # [inline] pub fn key (& self) -> & K { self . vacant . key () } # [doc = " Sets the value of the entry with the `VacantEntry`'s key, and returns a"] # [doc = " mutable reference to it."] # [inline] pub fn insert (self , value : V) -> & 'a mut V { self . vacant . insert (value) } }
    };
}

impl_26!()