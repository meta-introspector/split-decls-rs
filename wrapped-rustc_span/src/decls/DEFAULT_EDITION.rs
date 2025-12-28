macro_rules! deps {
    () => {
        Edition!();
    };
}

macro_rules! DEFAULT_EDITION {
    () => {
        deps!();
        pub const DEFAULT_EDITION : Edition = Edition :: Edition2015 ;
    };
}

DEFAULT_EDITION!()