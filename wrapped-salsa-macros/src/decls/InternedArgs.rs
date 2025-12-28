macro_rules! deps {
    () => {
        Options!();
        InternedStruct!();
    };
}

macro_rules! InternedArgs {
    () => {
        deps!();
        type InternedArgs = Options < InternedStruct > ;
    };
}

InternedArgs!()