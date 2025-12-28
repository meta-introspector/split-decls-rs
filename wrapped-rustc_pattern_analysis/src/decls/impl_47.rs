macro_rules! deps {
    () => {
        DeconstructedPat!();
        PatCx!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        # [doc = " Delegate to `uid`."] impl < Cx : PatCx > PartialEq for DeconstructedPat < Cx > { fn eq (& self , other : & Self) -> bool { self . uid == other . uid } }
    };
}

impl_47!();