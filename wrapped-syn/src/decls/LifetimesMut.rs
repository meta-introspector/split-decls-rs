macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! LifetimesMut {
    () => {
        deps!();
        pub struct LifetimesMut < 'a > (IterMut < 'a , GenericParam >) ;
    };
}

LifetimesMut!();