macro_rules! deps {
    () => {
        Decompositions!();
        RecompositionState!();
    };
}

macro_rules! Recompositions {
    () => {
        deps!();
        # [doc = " External iterator for a string recomposition's characters."] # [derive (Clone)] pub struct Recompositions < I > { iter : Decompositions < I > , state : RecompositionState , buffer : TinyVec < [char ; 4] > , composee : Option < char > , last_ccc : Option < u8 > , }
    };
}

Recompositions!();