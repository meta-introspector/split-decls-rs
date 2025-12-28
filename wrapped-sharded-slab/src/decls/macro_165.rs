macro_rules! deps {
    () => {
        Registration!();
    };
}

macro_rules! macro_165 {
    () => {
        deps!();
        thread_local ! { static REGISTRATION : Registration = Registration :: new () ; }
    };
}

macro_165!();