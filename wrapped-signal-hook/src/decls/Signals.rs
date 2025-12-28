macro_rules! deps {
    () => {
        SignalsInfo!();
        Exfiltrator!();
        SignalOnly!();
    };
}

macro_rules! Signals {
    () => {
        deps!();
        # [doc = " A type alias for an iterator returning just the signal numbers."] # [doc = ""] # [doc = " This is the simplified version for most of the use cases. For advanced usages, the"] # [doc = " [`SignalsInfo`] with explicit [`Exfiltrator`] type can be used."] pub type Signals = SignalsInfo < SignalOnly > ;
    };
}

Signals!()