macro_rules! deps {
    () => {
        InputStruct!();
        Options!();
    };
}

macro_rules! InputArgs {
    () => {
        deps!();
        type InputArgs = Options < InputStruct > ;
    };
}

InputArgs!();