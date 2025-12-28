macro_rules! deps {
    () => {
        TrackedStruct!();
        Options!();
    };
}

macro_rules! TrackedArgs {
    () => {
        deps!();
        type TrackedArgs = Options < TrackedStruct > ;
    };
}

TrackedArgs!()