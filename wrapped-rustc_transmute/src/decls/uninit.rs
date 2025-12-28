macro_rules! deps {
    () => {
        Assume!();
        Answer!();
        Tree!();
        Reason!();
    };
}

macro_rules! uninit {
    () => {
        deps!();
        mod uninit { use super :: * ; # [test] fn size () { let mu = Tree :: uninit () ; let u8 = Tree :: u8 () ; for alignment in [false , true] { for lifetimes in [false , true] { for safety in [false , true] { for validity in [false , true] { let assume = Assume { alignment , lifetimes , safety , validity } ; let want = if validity { Answer :: Yes } else { Answer :: No (Reason :: DstIsBitIncompatible) } ; assert_eq ! (is_transmutable (& mu , & u8 , assume) , want , "assume: {assume:?}") ; assert_eq ! (is_transmutable (& u8 , & mu , assume) , Answer :: Yes , "assume: {assume:?}") ; } } } } } }
    };
}

uninit!();