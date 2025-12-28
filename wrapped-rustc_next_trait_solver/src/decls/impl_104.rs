macro_rules! deps {
    () => {
        WipProbeStep!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < I : Interner > WipProbeStep < I > { fn finalize (self) -> inspect :: ProbeStep < I > { match self { WipProbeStep :: AddGoal (source , goal) => inspect :: ProbeStep :: AddGoal (source , goal) , WipProbeStep :: NestedProbe (probe) => inspect :: ProbeStep :: NestedProbe (probe . finalize ()) , WipProbeStep :: RecordImplArgs { impl_args } => { inspect :: ProbeStep :: RecordImplArgs { impl_args } } WipProbeStep :: MakeCanonicalResponse { shallow_certainty } => { inspect :: ProbeStep :: MakeCanonicalResponse { shallow_certainty } } } } }
    };
}

impl_104!();