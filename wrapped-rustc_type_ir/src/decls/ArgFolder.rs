macro_rules! deps {
    () => {
        Interner!();
        GenericArg!();
    };
}

macro_rules! ArgFolder {
    () => {
        deps!();
        struct ArgFolder < 'a , I : Interner > { cx : I , args : & 'a [I :: GenericArg] , # [doc = " Number of region binders we have passed through while doing the instantiation"] binders_passed : u32 , }
    };
}

ArgFolder!();