macro_rules! deps {
    () => {
        Answer!();
        MaybeTransmutableQuery!();
        Tree!();
        Reason!();
        Region!();
        Dfa!();
        Def!();
        QueryContext!();
        Type!();
        Uninhabited!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < C > MaybeTransmutableQuery < Tree < < C as QueryContext > :: Def , < C as QueryContext > :: Region , < C as QueryContext > :: Type > , C , > where C : QueryContext , { # [doc = " Answers whether a `Tree` is transmutable into another `Tree`."] # [doc = ""] # [doc = " This method begins by de-def'ing `src` and `dst`, and prunes private paths from `dst`,"] # [doc = " then converts `src` and `dst` to `Dfa`s, and computes an answer using those DFAs."] # [inline (always)] # [instrument (level = "debug" , skip (self) , fields (src = ? self . src , dst = ? self . dst))] pub (crate) fn answer (self) -> Answer < < C as QueryContext > :: Region , < C as QueryContext > :: Type > { let Self { src , dst , assume , context } = self ; let src = src . prune (& | _def | false) ; if src . is_inhabited () && ! dst . is_inhabited () { return Answer :: No (Reason :: DstUninhabited) ; } trace ! (? src , "pruned src") ; let dst = if assume . safety { dst . prune (& | _def | false) } else { dst . prune (& | def | def . has_safety_invariants ()) } ; trace ! (? dst , "pruned dst") ; let src = match Dfa :: from_tree (src) { Ok (src) => src , Err (layout :: Uninhabited) => return Answer :: Yes , } ; let dst = match Dfa :: from_tree (dst) { Ok (dst) => dst , Err (layout :: Uninhabited) => return Answer :: No (Reason :: DstMayHaveSafetyInvariants) , } ; MaybeTransmutableQuery { src , dst , assume , context } . answer () } }
    };
}

impl_63!()