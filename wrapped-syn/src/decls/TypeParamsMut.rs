macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! TypeParamsMut {
    () => {
        deps!();
        pub struct TypeParamsMut < 'a > (IterMut < 'a , GenericParam >) ;
    };
}

TypeParamsMut!()