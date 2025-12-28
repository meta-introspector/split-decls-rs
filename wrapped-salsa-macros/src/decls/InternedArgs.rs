macro_rules! deps {
    () => {
        InternedStruct!();
        Options!();
    };
}

macro_rules! InternedArgs {
    () => {
        deps!();
        type InternedArgs = Options < InternedStruct > ;
    };
}

InternedArgs!();