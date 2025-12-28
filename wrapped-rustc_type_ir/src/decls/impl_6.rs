macro_rules! deps {
    () => {
        DelayedMap!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < K : Hash + Eq , V > DelayedMap < K , V > { # [inline (always)] pub fn insert (& mut self , key : K , value : V) -> bool { if self . count >= CACHE_CUTOFF { self . cold_insert (key , value) } else { self . count += 1 ; true } } # [cold] # [inline (never)] fn cold_insert (& mut self , key : K , value : V) -> bool { self . cache . insert (key , value) . is_none () } # [inline (always)] pub fn get (& self , key : & K) -> Option < & V > { if self . cache . is_empty () { None } else { self . cold_get (key) } } # [cold] # [inline (never)] fn cold_get (& self , key : & K) -> Option < & V > { self . cache . get (key) } }
    };
}

impl_6!()