macro_rules! deps {
    () => {
        DeconstructedPat!();
        PatCx!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        # [doc = " Delegate to `uid`."] impl < Cx : PatCx > Eq for DeconstructedPat < Cx > { }
    };
}

impl_48!();