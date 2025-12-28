macro_rules! WIDTH {
    () => {
        pub (crate) const WIDTH : usize = std :: mem :: size_of :: < usize > () * 8 ;
    };
}

WIDTH!();