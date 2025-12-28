macro_rules! deps {
    () => {
        GenericArg!();
        Interner!();
    };
}

macro_rules! TypeWalkerStack {
    () => {
        deps!();
        type TypeWalkerStack < I > = SmallVec < [< I as Interner > :: GenericArg ; 8] > ;
    };
}

TypeWalkerStack!()