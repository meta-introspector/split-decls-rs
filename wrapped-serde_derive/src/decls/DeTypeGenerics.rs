macro_rules! deps {
    () => {
        Parameters!();
    };
}

macro_rules! DeTypeGenerics {
    () => {
        deps!();
        struct DeTypeGenerics < 'a > (& 'a Parameters) ;
    };
}

DeTypeGenerics!();