macro_rules! deps {
    () => {
        DelayedSet!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < T : Hash + Eq > DelayedSet < T > { # [inline (always)] pub fn insert (& mut self , value : T) -> bool { if self . count >= CACHE_CUTOFF { self . cold_insert (value) } else { self . count += 1 ; true } } # [cold] # [inline (never)] fn cold_insert (& mut self , value : T) -> bool { self . cache . insert (value) } # [inline (always)] pub fn contains (& self , value : & T) -> bool { ! self . cache . is_empty () && self . cold_contains (value) } # [cold] # [inline (never)] fn cold_contains (& self , value : & T) -> bool { self . cache . contains (value) } }
    };
}

impl_9!();