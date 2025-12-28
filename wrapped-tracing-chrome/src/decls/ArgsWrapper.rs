macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! ArgsWrapper {
    () => {
        deps!();
        struct ArgsWrapper { args : Arc < Object > , }
    };
}

ArgsWrapper!()