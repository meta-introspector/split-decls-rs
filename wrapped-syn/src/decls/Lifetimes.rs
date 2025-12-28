macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! Lifetimes {
    () => {
        deps!();
        pub struct Lifetimes < 'a > (Iter < 'a , GenericParam >) ;
    };
}

Lifetimes!()