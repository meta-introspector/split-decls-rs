macro_rules! deps {
    () => {
        Entry!();
        SsoHashMap!();
    };
}

macro_rules! impl_455 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , V > Entry < 'a , K , V > { # [doc = " Provides in-place mutable access to an occupied entry before any"] # [doc = " potential inserts into the map."] pub fn and_modify < F > (self , f : F) -> Self where F : FnOnce (& mut V) , { if let Some (value) = self . ssomap . get_mut (& self . key) { f (value) ; } self } # [doc = " Ensures a value is in the entry by inserting the default if empty, and returns"] # [doc = " a mutable reference to the value in the entry."] # [inline] pub fn or_insert (self , value : V) -> & 'a mut V { self . or_insert_with (| | value) } # [doc = " Ensures a value is in the entry by inserting the result of the default function if empty,"] # [doc = " and returns a mutable reference to the value in the entry."] pub fn or_insert_with < F : FnOnce () -> V > (self , default : F) -> & 'a mut V { self . ssomap . migrate_if_full () ; match self . ssomap { SsoHashMap :: Array (array) => { let key_ref = & self . key ; let found_index = array . iter () . position (| (k , _v) | k == key_ref) ; let index = if let Some (index) = found_index { index } else { let index = array . len () ; array . try_push ((self . key , default ())) . unwrap () ; index } ; & mut array [index] . 1 } SsoHashMap :: Map (map) => map . entry (self . key) . or_insert_with (default) , } } # [doc = " Returns a reference to this entry's key."] # [inline] pub fn key (& self) -> & K { & self . key } }
    };
}

impl_455!()