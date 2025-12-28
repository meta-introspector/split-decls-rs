macro_rules! deps {
    () => {
        SignalsInfo!();
    };
}

macro_rules! Signals {
    () => {
        deps!();
        # [doc = " Simplified version of the signals stream."] # [doc = ""] # [doc = " This one simply returns the signal numbers, while [`SignalsInfo`] can provide additional"] # [doc = " information."] pub type Signals = SignalsInfo < SignalOnly > ;
    };
}

Signals!()