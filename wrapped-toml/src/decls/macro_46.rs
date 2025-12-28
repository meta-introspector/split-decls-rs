macro_rules! deps {
    () => {
        Keys!();
    };
}

macro_rules! macro_46 {
    () => {
        deps!();
        delegate_iterator ! ((Keys <'a , K , V >) => &'a K) ;
    };
}

macro_46!()