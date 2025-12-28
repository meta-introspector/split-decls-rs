macro_rules! deps {
    () => {
        Exfiltrator!();
        PendingSignals!();
    };
}

macro_rules! Pending {
    () => {
        deps!();
        # [doc = " The iterator of one batch of signals."] # [doc = ""] # [doc = " This is returned by the [`pending`][SignalDelivery::pending] method."] # [derive (Debug)] pub struct Pending < E : Exfiltrator > { pending : Arc < PendingSignals < E > > , position : usize , }
    };
}

Pending!();