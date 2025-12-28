macro_rules! deps {
    () => {
        Options!();
        TrackedFn!();
    };
}

macro_rules! FnArgs {
    () => {
        deps!();
        pub type FnArgs = Options < TrackedFn > ;
    };
}

FnArgs!();