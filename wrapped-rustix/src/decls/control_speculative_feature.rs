macro_rules! deps {
    () => {
        SpeculationFeature!();
        Result!();
    };
}

macro_rules! control_speculative_feature {
    () => {
        deps!();
        # [doc = " Sets the state of the speculation misfeature."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_SPECULATION_CTRL,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_SPECULATION_CTRL,…)`]: https://www.kernel.org/doc/html/v6.13/userspace-api/spec_ctrl.html"] # [inline] # [doc (alias = "PR_SET_SPECULATION_CTRL")] pub fn control_speculative_feature (feature : SpeculationFeature , config : SpeculationFeatureControl ,) -> io :: Result < () > { let feature = feature as usize as * mut _ ; let config = config . bits () as usize as * mut _ ; unsafe { prctl_3args (PR_SET_SPECULATION_CTRL , feature , config) } . map (| _r | ()) }
    };
}

control_speculative_feature!();