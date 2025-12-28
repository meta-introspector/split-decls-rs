macro_rules! deps {
    () => {
        SortedMap!();
    };
}

macro_rules! impl_428 {
    () => {
        deps!();
        impl < K , V > Default for SortedMap < K , V > { # [inline] fn default () -> SortedMap < K , V > { SortedMap { data : Vec :: new () } } }
    };
}

impl_428!()