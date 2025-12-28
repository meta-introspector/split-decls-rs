macro_rules! deps {
    () => {
        Exfiltrator!();
        Handle!();
        PendingSignals!();
    };
}

macro_rules! SignalDelivery {
    () => {
        deps!();
        # [doc = " A struct for delivering received signals to the main program flow."] # [doc = " The self-pipe IO type is generic. See the"] # [doc = " [`with_pipe`][SignalDelivery::with_pipe] method for requirements"] # [doc = " for the IO type."] # [derive (Debug)] pub struct SignalDelivery < R , E : Exfiltrator > { read : R , handle : Handle , pending : Arc < PendingSignals < E > > , }
    };
}

SignalDelivery!()