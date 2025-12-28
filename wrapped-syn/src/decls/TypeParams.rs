macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! TypeParams {
    () => {
        deps!();
        pub struct TypeParams < 'a > (Iter < 'a , GenericParam >) ;
    };
}

TypeParams!();