macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! macro_37 {
    () => {
        deps!();
        delegate_iterator ! ((IterMut <'a , K , V >) => (&'a K , &'a mut V)) ;
    };
}

macro_37!();