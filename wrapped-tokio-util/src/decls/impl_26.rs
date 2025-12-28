macro_rules! deps {
    () => {
        JoinMapKeys!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < 'a , K , V > std :: iter :: FusedIterator for JoinMapKeys < 'a , K , V > { }
    };
}

impl_26!()