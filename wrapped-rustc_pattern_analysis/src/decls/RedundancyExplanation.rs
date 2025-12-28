macro_rules! deps {
    () => {
        DeconstructedPat!();
        PatCx!();
    };
}

macro_rules! RedundancyExplanation {
    () => {
        deps!();
        # [doc = " Indicates why a given pattern is considered redundant."] # [derive (Clone , Debug)] pub struct RedundancyExplanation < 'p , Cx : PatCx > { # [doc = " All the values matched by this pattern are already matched by the given set of patterns."] # [doc = " This list is not guaranteed to be minimal but the contained patterns are at least guaranteed"] # [doc = " to intersect this pattern."] pub covered_by : Vec < & 'p DeconstructedPat < Cx > > , }
    };
}

RedundancyExplanation!();