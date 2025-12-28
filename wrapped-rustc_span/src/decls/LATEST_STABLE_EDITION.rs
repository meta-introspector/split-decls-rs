macro_rules! deps {
    () => {
        Edition!();
    };
}

macro_rules! LATEST_STABLE_EDITION {
    () => {
        deps!();
        pub const LATEST_STABLE_EDITION : Edition = Edition :: Edition2024 ;
    };
}

LATEST_STABLE_EDITION!()