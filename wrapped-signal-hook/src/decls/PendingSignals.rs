macro_rules! deps {
    () => {
        Exfiltrator!();
    };
}

macro_rules! PendingSignals {
    () => {
        deps!();
        struct PendingSignals < E : Exfiltrator > { exfiltrator : E , slots : [E :: Storage ; MAX_SIGNUM] , }
    };
}

PendingSignals!()