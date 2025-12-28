macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! macro_42 {
    () => {
        deps!();
        delegate_iterator ! ((IntoIter < K , V >) => (K , V)) ;
    };
}

macro_42!();