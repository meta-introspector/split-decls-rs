macro_rules! deps {
    () => {
        Options!();
        InputStruct!();
    };
}

macro_rules! InputArgs {
    () => {
        deps!();
        type InputArgs = Options < InputStruct > ;
    };
}

InputArgs!()