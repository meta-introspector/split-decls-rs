macro_rules! deps {
    () => {
        SortedMap!();
    };
}

macro_rules! impl_429 {
    () => {
        deps!();
        impl < K , V > SortedMap < K , V > { # [inline] pub const fn new () -> SortedMap < K , V > { SortedMap { data : Vec :: new () } } }
    };
}

impl_429!()