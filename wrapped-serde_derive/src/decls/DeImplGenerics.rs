macro_rules! deps {
    () => {
        Parameters!();
    };
}

macro_rules! DeImplGenerics {
    () => {
        deps!();
        struct DeImplGenerics < 'a > (& 'a Parameters) ;
    };
}

DeImplGenerics!();