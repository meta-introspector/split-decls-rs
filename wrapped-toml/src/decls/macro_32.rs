macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! macro_32 {
    () => {
        deps!();
        delegate_iterator ! ((Iter <'a , K , V >) => (&'a K , &'a V)) ;
    };
}

macro_32!()