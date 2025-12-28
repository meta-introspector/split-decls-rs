macro_rules! deps {
    () => {
        VerboseInfo!();
        TimingGuard!();
    };
}

macro_rules! VerboseTimingGuard {
    () => {
        deps!();
        # [must_use] pub struct VerboseTimingGuard < 'a > { info : Option < VerboseInfo > , _guard : TimingGuard < 'a > , }
    };
}

VerboseTimingGuard!();