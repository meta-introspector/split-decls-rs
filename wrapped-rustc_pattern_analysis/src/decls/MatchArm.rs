macro_rules! deps {
    () => {
        PatCx!();
    };
}

macro_rules! MatchArm {
    () => {
        deps!();
        # [doc = " The arm of a match expression."] # [derive (Debug)] pub struct MatchArm < 'p , Cx : PatCx > { pub pat : & 'p DeconstructedPat < Cx > , pub has_guard : bool , pub arm_data : Cx :: ArmData , }
    };
}

MatchArm!()