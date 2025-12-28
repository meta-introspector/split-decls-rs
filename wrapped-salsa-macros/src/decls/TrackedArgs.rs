macro_rules! deps {
    () => {
        Options!();
        TrackedStruct!();
    };
}

macro_rules! TrackedArgs {
    () => {
        deps!();
        type TrackedArgs = Options < TrackedStruct > ;
    };
}

TrackedArgs!();