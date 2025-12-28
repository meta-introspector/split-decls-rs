macro_rules! deps {
    () => {
        PatCx!();
        DeconstructedPat!();
    };
}

macro_rules! IndexedPat {
    () => {
        deps!();
        # [doc = " A pattern with an index denoting which field it corresponds to."] pub struct IndexedPat < Cx : PatCx > { pub idx : usize , pub pat : DeconstructedPat < Cx > , }
    };
}

IndexedPat!();