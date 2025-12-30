// Generated macro for impl_92 (impl)
macro_rules! Depcrate_maybe_transmutableimpl_92 {
() => {
// Module: crate::maybe_transmutable
// Provides: {"impl_92"}
// Dependencies: {}
impl Quantifier { fn apply < R , T , I > (& self , iter : I) -> Answer < R , T > where R : layout :: Region , T : layout :: Type , I : IntoIterator < Item = Answer < R , T > > , { use std :: ops :: ControlFlow :: { Break , Continue } ; let (init , try_fold_f) : (_ , fn (_ , _) -> _) = match self { Self :: ThereExists => { (Answer :: No (Reason :: DstIsBitIncompatible) , | accum : Answer < R , T > , next | match accum . or (next) { Answer :: Yes => Break (Answer :: Yes) , maybe => Continue (maybe) , }) } Self :: ForAll => (Answer :: Yes , | accum : Answer < R , T > , next | { let answer = accum . and (next) ; match answer { Answer :: No (_) => Break (answer) , maybe => Continue (maybe) , } }) , } ; let (Continue (result) | Break (result)) = iter . into_iter () . try_fold (init , try_fold_f) ; result } }
};
}
