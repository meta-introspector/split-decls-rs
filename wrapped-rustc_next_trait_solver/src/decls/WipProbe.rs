macro_rules! deps {
    () => {
        WipProbeStep!();
    };
}

macro_rules! WipProbe {
    () => {
        deps!();
        # [derive_where (PartialEq , Debug ; I : Interner)] struct WipProbe < I : Interner > { initial_num_var_values : usize , steps : Vec < WipProbeStep < I > > , kind : Option < inspect :: ProbeKind < I > > , final_state : Option < inspect :: CanonicalState < I , () > > , }
    };
}

WipProbe!();