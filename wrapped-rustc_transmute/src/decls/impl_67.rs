macro_rules! deps {
    () => {
        Answer!();
        Quantifier!();
        Region!();
        Type!();
        Reason!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl Quantifier { fn apply < R , T , I > (& self , iter : I) -> Answer < R , T > where R : layout :: Region , T : layout :: Type , I : IntoIterator < Item = Answer < R , T > > , { use std :: ops :: ControlFlow :: { Break , Continue } ; let (init , try_fold_f) : (_ , fn (_ , _) -> _) = match self { Self :: ThereExists => { (Answer :: No (Reason :: DstIsBitIncompatible) , | accum : Answer < R , T > , next | match accum . or (next) { Answer :: Yes => Break (Answer :: Yes) , maybe => Continue (maybe) , }) } Self :: ForAll => (Answer :: Yes , | accum : Answer < R , T > , next | { let answer = accum . and (next) ; match answer { Answer :: No (_) => Break (answer) , maybe => Continue (maybe) , } }) , } ; let (Continue (result) | Break (result)) = iter . into_iter () . try_fold (init , try_fold_f) ; result } }
    };
}

impl_67!()