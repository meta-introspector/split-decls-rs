macro_rules! deps {
    () => {
        Exfiltrator!();
        Pending!();
    };
}

macro_rules! SignalIterator {
    () => {
        deps!();
        # [doc = " An infinite iterator of received signals."] pub struct SignalIterator < SD , E : Exfiltrator > { signals : SD , iter : Pending < E > , }
    };
}

SignalIterator!()