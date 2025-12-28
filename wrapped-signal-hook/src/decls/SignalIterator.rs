macro_rules! deps {
    () => {
        Pending!();
        Exfiltrator!();
    };
}

macro_rules! SignalIterator {
    () => {
        deps!();
        # [doc = " An infinite iterator of received signals."] pub struct SignalIterator < SD , E : Exfiltrator > { signals : SD , iter : Pending < E > , }
    };
}

SignalIterator!();