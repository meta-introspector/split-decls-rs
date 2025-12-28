macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! ConstParamsMut {
    () => {
        deps!();
        pub struct ConstParamsMut < 'a > (IterMut < 'a , GenericParam >) ;
    };
}

ConstParamsMut!()