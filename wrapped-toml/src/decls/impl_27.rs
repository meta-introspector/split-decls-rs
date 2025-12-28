macro_rules! deps {
    () => {
        OccupiedEntry!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < 'a , K : Ord , V > OccupiedEntry < 'a , K , V > { # [doc = " Gets a reference to the key in the entry."] # [inline] pub fn key (& self) -> & K { self . occupied . key () } # [doc = " Gets a reference to the value in the entry."] # [inline] pub fn get (& self) -> & V { self . occupied . get () } # [doc = " Gets a mutable reference to the value in the entry."] # [inline] pub fn get_mut (& mut self) -> & mut V { self . occupied . get_mut () } # [doc = " Converts the entry into a mutable reference to its value."] # [inline] pub fn into_mut (self) -> & 'a mut V { self . occupied . into_mut () } # [doc = " Sets the value of the entry with the `OccupiedEntry`'s key, and returns"] # [doc = " the entry's old value."] # [inline] pub fn insert (& mut self , value : V) -> V { self . occupied . insert (value) } # [doc = " Takes the value of the entry out of the map, and returns it."] # [inline] pub fn remove (self) -> V { # [cfg (not (feature = "preserve_order"))] { self . occupied . remove () } # [cfg (feature = "preserve_order")] { self . occupied . shift_remove () } } }
    };
}

impl_27!();