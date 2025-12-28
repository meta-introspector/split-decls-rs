macro_rules! deps {
    () => {
        Edition!();
    };
}

macro_rules! ALL_EDITIONS {
    () => {
        deps!();
        pub const ALL_EDITIONS : & [Edition] = & [Edition :: Edition2015 , Edition :: Edition2018 , Edition :: Edition2021 , Edition :: Edition2024 , Edition :: EditionFuture ,] ;
    };
}

ALL_EDITIONS!()