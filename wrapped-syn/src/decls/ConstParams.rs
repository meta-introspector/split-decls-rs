macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! ConstParams {
    () => {
        deps!();
        pub struct ConstParams < 'a > (Iter < 'a , GenericParam >) ;
    };
}

ConstParams!();