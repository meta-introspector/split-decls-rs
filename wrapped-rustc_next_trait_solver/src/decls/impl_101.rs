macro_rules! deps {
    () => {
        WipProbeStep!();
        WipProbe!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < I : Interner > WipProbe < I > { fn finalize (self) -> inspect :: Probe < I > { inspect :: Probe { steps : self . steps . into_iter () . map (WipProbeStep :: finalize) . collect () , kind : self . kind . unwrap () , final_state : self . final_state . unwrap () , } } }
    };
}

impl_101!();