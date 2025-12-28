macro_rules! NonEmptyVec {
    () => {
        # [doc = " A [`Vec`] that is guaranteed to at least contain one element."] pub struct NonEmptyVec < T > { first : T , rest : Vec < T > , }
    };
}

NonEmptyVec!();