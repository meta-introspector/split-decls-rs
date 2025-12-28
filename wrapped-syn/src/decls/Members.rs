macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! Members {
    () => {
        deps!();
        pub struct Members < 'a > { fields : punctuated :: Iter < 'a , Field > , index : u32 , }
    };
}

Members!();