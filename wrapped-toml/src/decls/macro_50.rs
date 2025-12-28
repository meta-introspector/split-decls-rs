macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! macro_50 {
    () => {
        deps!();
        delegate_iterator ! ((Values <'a , K , V >) => &'a V) ;
    };
}

macro_50!();